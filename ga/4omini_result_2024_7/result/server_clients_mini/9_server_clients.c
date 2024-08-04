use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write, Read};
use std::os::unix::prelude::*;
use std::mem;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap2::MmapMut;

fn main() -> io::Result<()> {
    // Create and open a file
    let file_path = "temp";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // Set permissions
        .open(file_path)?;

    // Set the size of the file
    file.set_len(4096)?;

    // Memory-map the file
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut mmap[1024..];
        let message = b"Test message";

        // Copy the message to the memory location
        p[..message.len()].copy_from_slice(message);
        println!("Message written to the shared memory: {:?}", String::from_utf8_lossy(&p[..message.len()]));

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset 
        println!("Read from shared memory: {:?}", String::from_utf8_lossy(&mmap[1024..message.len() + 1024]));
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // No need to explicitly unmap since Rust and Mmap handles that
    // Close the file
    drop(mmap); // Explicitly drop to ensure mmap is released before the file is dropped

    // Remove the temporary file
    remove_file(file_path)?;

    Ok(())
}