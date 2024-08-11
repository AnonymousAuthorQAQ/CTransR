use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
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
        let msg = b"Test message\x00";
        unsafe {
            ptr::copy_nonoverlapping(msg.as_ptr(), addr.offset(1024), msg.len());
        }

        let msg = unsafe {
            let mut buf = [0; 11];
            ptr::copy_nonoverlapping(addr.offset(1024), buf.as_mut_ptr(), msg.len());
            str::from_utf8(&buf).unwrap()
        };

        println!("Message written to the shared memory: {}", msg);
        sleep(Duration::from_secs(1));

        let msg = unsafe {
            let mut buf = [0; 11];
            ptr::copy_nonoverlapping(addr.offset(1024), buf.as_mut_ptr(), msg.len());
            str::from_utf8(&buf).unwrap()
        };

        println!("Read from shared memory: {}", msg);
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        if libc::munmap(addr, 4096) == -1 {
            eprintln!("Failed to munmap");
        }
    }

    file.sync_all().expect("Failed to sync file");
}