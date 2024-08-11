use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::mem::size_of;
use std::ptr::null_mut;
use libc::{c_void, c_int, off_t, MAP_FAILED, PROT_READ, PROT_WRITE, MAP_SHARED, ftruncate, mmap, munmap};

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
    let size: off_t = 4096;
    unsafe {
        if ftruncate(fd, size) == -1 {
            panic!("Failed to truncate file");
        }

        // Map the file into memory
        let ptr = mmap(null_mut(), size_of::<c_int>() as usize, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
        if ptr == MAP_FAILED {
            panic!("Failed to mmap");
        }

        let ptr = ptr as *mut c_void;

        // Main loop
        let mut stop = 0;
        while stop < 10 {
            // Write to a specific offset within the memory-mapped area
            let p = (ptr as *mut u8).offset(1024);
            let message = b"Test message";
            std::ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
            println!("Message written to the shared memory: {}", std::str::from_utf8_unchecked(std::slice::from_raw_parts(p, message.len())));

            // Read from the same offset
            let read_message = std::slice::from_raw_parts(p, message.len());
            println!("Read from shared memory: {}", std::str::from_utf8_unchecked(read_message));

            stop += 1;
        }

        // Unmap the memory
        if munmap(ptr, size_of::<c_int>() as usize) == -1 {
            panic!("Failed to munmap");
        }
    }
}