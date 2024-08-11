use std::ffi::CString;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void};
use std::fs::OpenOptions;
use std::os::unix::io::{AsRawFd, RawFd};
use std::slice;
use std::mem;
use std::{thread, time};

fn main() {
    // Create and open a file
    let file = OpenOptions::new().read(true).write(true).create(true).open("temp").unwrap();
    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        if libc::ftruncate(fd, 4096) == -1 {
            panic!("Failed to truncate file size");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0)
    };
    if ptr == libc::MAP_FAILED {
        panic!("Failed to map file into memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        unsafe {
            // Write to a specific offset within the memory-mapped area
            let p = ptr.offset(1024);
            let message = CString::new("Test message").unwrap().as_ptr();
            libc::strcpy(p as *mut i8, message);
            println!("Message written to the shared memory: {}", CString::from_raw(p as *mut i8).to_string_lossy());

            thread::sleep(time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

            // Read from the same offset
            println!("Read from shared memory: {}", CString::from_raw(p as *mut i8).to_string_lossy());

            thread::sleep(time::Duration::from_secs(1)); // Sleep again to simulate delay

            // Increment counter
            stop += 1;
        }
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, 4096) == -1 {
            panic!("Failed to unmap memory");
        }
    }

    drop(file); // Close the file

    // Delete the file
    if let Err(e) = std::fs::remove_file("temp") {
        panic!("Failed to delete file: {}", e);
    }
}