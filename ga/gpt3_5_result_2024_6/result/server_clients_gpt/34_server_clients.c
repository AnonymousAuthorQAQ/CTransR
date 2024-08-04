use std::ffi::CString;
use std::os::raw::{c_void, c_char};
use std::{ptr, mem};
use std::fs::{OpenOptions, File};
use std::os::unix::io::AsRawFd;
use std::io::Write;

extern crate libc;
use libc::{c_int, c_uint, size_t, off_t};

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
    let file_size: off_t = 4096;
    unsafe {
        libc::ftruncate(fd, file_size);
    }

    // Map the file into memory
    let ptr: *mut c_void = unsafe {
        libc::mmap(
            ptr::null_mut(),
            file_size as size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        panic!("Failed to map the file into memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let p = (ptr as *mut c_char).offset(offset as isize);
        let message = CString::new("Test message").expect("CString conversion failed");
        unsafe {
            libc::strcpy(p, message.as_ptr());
        }

        println!(
            "Message written to the shared memory: {}",
            std::str::from_utf8(std::ffi::CStr::from_ptr(p).to_bytes()).unwrap()
        );

        std::thread::sleep(std::time::Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(std::ffi::CStr::from_ptr(p).to_bytes()).unwrap());
        
        std::thread::sleep(std::time::Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(ptr, file_size as size_t);
    }

    drop(file);
}