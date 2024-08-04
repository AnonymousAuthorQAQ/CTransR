use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::RawFd;
use std::ptr;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Open failed");

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        libc::ftruncate(fd, 4096);
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).add(1024);
        let message = CString::new("Test message").expect("CString creation failed");
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", std::str::from_utf8(unsafe {
            std::slice::from_raw_parts(p, message.as_bytes().len())
        }).expect("Conversion failed"));
        
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(unsafe {
            std::slice::from_raw_parts((ptr as *mut u8).add(1024), message.as_bytes().len())
        }).expect("Conversion failed"));

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(ptr, 4096);
    }

    drop(file);

    std::fs::remove_file("temp").expect("Failed to delete file");
}