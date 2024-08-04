use std::ffi::CString;
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        if libc::ftruncate(fd, 4096) == -1 {
            panic!("Failed to set file size");
        }
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
        panic!("Failed to map file into memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).add(1024) };
        let message = b"Test message\0";
        unsafe {
            libc::strcpy(p as *mut libc::c_char, message.as_ptr() as *const libc::c_char);
        }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const libc::c_char).to_str().unwrap()
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!(
            "Read from shared memory: {}",
            unsafe { std::ffi::CStr::from_ptr(p as *const libc::c_char).to_str().unwrap() }
        );

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, 4096) == -1 {
            panic!("Failed to unmap memory");
        }
    }

    drop(file);

    if let Err(_) = std::fs::remove_file("temp") {
        panic!("Failed to delete file");
    }
}