use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
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
        .expect("Error opening file");

    file.set_len(4096).expect("Error setting file length");

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
        panic!("Error mmapping the file");
    }

    let mut stop = 0;
    while stop < 10 {
        unsafe {
            let p = slice::from_raw_parts_mut(ptr.offset(1024), 11);
            p.copy_from_slice(b"Test message");
        }

        let message = unsafe {
            let p = slice::from_raw_parts(ptr.offset(1024), 11);
            str::from_utf8(p).unwrap()
        };
        println!("Message written to the shared memory: {}", message);

        sleep(Duration::from_secs(1));

        let message = unsafe {
            let p = slice::from_raw_parts(ptr.offset(1024), 11);
            str::from_utf8(p).unwrap()
        };
        println!("Read from shared memory: {}", message);

        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        libc::munmap(ptr, 4096);
    }

    file.sync_all().expect("Error syncing the file");
}
