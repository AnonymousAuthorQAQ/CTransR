use std::fs::{OpenOptions, File};
use std::io::{Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::mem;
use libc::{c_void, MAP_SHARED, PROT_READ, PROT_WRITE, MAP_FAILED, ftruncate, fcntl, F_TRUNC, O_RDWR, O_CREAT, close, unlink, munmap, PROT_EXEC, PROT_SEM};
use nix::unistd::sleep;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to create file");

    // Set the size of the file
    unsafe {
        if ftruncate(file.as_raw_fd(), 4096) == -1 {
            panic!("Failed to truncate file");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if ptr == MAP_FAILED {
        panic!("Failed to mmap");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p: *mut c_void = (ptr as *mut c_void).add(1024);
        let message = "Test message".as_bytes();
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr() as *const i8);
        }
        println!("Message written to the shared memory: {}", std::str::from_utf8(message).unwrap());

        sleep(1);

        // Read from the same offset
        let read_ptr = (ptr as *mut c_void).add(1024);
        let read_message = unsafe { std::ffi::CStr::from_ptr(read_ptr as *const i8).to_str().unwrap() };
        println!("Read from shared memory: {}", read_message);

        sleep(1);

        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("Failed to munmap");
        }
    }

    close(file.as_raw_fd());

    if unlink("temp") == -1 {
        panic!("Failed to unlink");
    }
}