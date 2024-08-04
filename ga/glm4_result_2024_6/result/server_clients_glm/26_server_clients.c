use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let path = Path::new("temp");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    file.set_len(4096).unwrap();

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    let mut stop = 0;
    while stop < 10 {
        unsafe {
            let p = slice::from_raw_parts_mut(ptr.offset(1024), 1024);
            p.copy_from_slice(b"Test message");
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(ptr.offset(1024), 1024))
        });
        sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(ptr.offset(1024), 1024))
        });
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        libc::munmap(ptr, 4096);
    }

    file.sync_all().unwrap();
    drop(file);
    std::fs::remove_file(path).unwrap();
}