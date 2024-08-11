use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::thread;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .unwrap();

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        libc::ftruncate(fd, 4096);
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0)
    };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);

        unsafe {
            libc::strcpy(p as *mut i8, CString::new("Test message").unwrap().as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe { CStr::from_ptr(p as *const i8).to_str().unwrap() });

        // Sleep to simulate delay and observe the output
        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { CStr::from_ptr(ptr as *const i8).to_str().unwrap() });

        // Sleep again to simulate delay
        thread::sleep(Duration::from_secs(1));

        stop += 1;
    }

    // Unmap memory
    unsafe {
        libc::munmap(ptr, 4096);
    }

    // Close file and remove it
    drop(file); // File will be closed when out of scope

    match std::fs::remove_file("temp") {
        Ok(_) => {},
        Err(e) => panic!("Error removing file: {}", e),
    }
}