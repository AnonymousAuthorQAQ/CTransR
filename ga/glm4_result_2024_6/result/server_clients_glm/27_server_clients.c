use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let path = Path::new("temp");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    file.set_len(4096).expect("Failed to set file length");

    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        eprintln!("Failed to mmap");
        return;
    }

    let mut stop = 0;
    while stop < 10 {
        let msg = b"Test message";
        unsafe {
            ptr::copy_nonoverlapping(msg.as_ptr(), addr as *mut u8, msg.len());
        }

        let read_msg = unsafe { str::from_utf8_unchecked(&*(addr as *const [u8; 11])) };
        println!("Message written to the shared memory: {}", read_msg);

        sleep(Duration::from_secs(1));

        let read_msg = unsafe { str::from_utf8_unchecked(&*(addr as *const [u8; 11])) };
        println!("Read from shared memory: {}", read_msg);

        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        libc::munmap(addr, 4096).expect("Failed to munmap");
    }

    file.sync_all().expect("Failed to sync file");
    std::fs::remove_file(path).expect("Failed to remove file");
}