use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

    // Memory map the file
    let mapping = unsafe {
        let addr = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );

        if addr == libc::MAP_FAILED {
            panic!("Failed to map memory");
        }

        addr
    };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message";
        unsafe {
            ptr::copy(message.as_ptr(), mapping.offset(1024), message.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            String::from_utf8_unchecked(slice::from_raw_parts(
                mapping.offset(1024),
                message.len(),
            ))
        });
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            String::from_utf8_unchecked(slice::from_raw_parts(
                mapping.offset(1024),
                message.len(),
            ))
        });
        sleep(Duration::from_secs(1));
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(mapping, 4096);
    }

    file.close().expect("Failed to close file");

    fs::remove_file("temp").expect("Failed to remove file");
}