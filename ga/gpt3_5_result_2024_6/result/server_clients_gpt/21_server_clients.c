use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use libc::{c_void, off_t, ftruncate, mmap, munmap, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED};

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    unsafe {
        let fd = file.as_raw_fd();
        let result = ftruncate(fd, 4096 as off_t);
        if result == -1 {
            panic!("Failed to truncate file");
        }

        // Map the file into memory
        let ptr = mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
        if ptr == MAP_FAILED {
            panic!("Failed to map file into memory");
        }

        // Main loop
        let mut stop = 0;
        while stop < 10 {
            // Write to a specific offset within the memory-mapped area
            let p = (ptr as *mut u8).offset(1024);
            let message = CString::new("Test message").unwrap().as_ptr();
            std::ptr::copy_nonoverlapping(message, p, 12); // Copy 12 bytes
            println!("Message written to the shared memory: {}", String::from_utf8_lossy(slice::from_raw_parts(p, 12)));
            thread::sleep(Duration::from_secs(1));

            // Read from the same offset
            println!("Read from shared memory: {}", String::from_utf8_lossy(slice::from_raw_parts(p, 12)));
            thread::sleep(Duration::from_secs(1));
            stop += 1;
        }

        // Unmap memory
        let result = munmap(ptr, 4096);
        if result == -1 {
            panic!("Failed to unmap memory");
        }
    }

    // Close the file
    // File is automatically closed when it goes out of scope
}