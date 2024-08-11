use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::mem;
use std::os::unix::fs::FileExt;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap::{MmapMut, MmapOptions}; // Add this crate to Cargo.toml

fn main() -> io::Result<()> {
    // Create and open a file
    let file_path = "temp";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap = unsafe { MmapOptions::new().len(4096).map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = "Test message";
        
        // Ensure the string fits within the boundary
        let len = message.len().min(mmap.len() - offset - 1);
        
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), mmap.as_mut_ptr().add(offset), len);
            *mmap.as_mut_ptr().add(offset + len) = 0; // Null-terminate the string
        }

        println!("Message written to the shared memory: {}", unsafe {
            std::str::from_utf8_unchecked(&mmap[offset..offset + len])
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::str::from_utf8_unchecked(&mmap[offset..offset + len])
        });

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap is handled automatically when `mmap` goes out of scope
    drop(mmap);
    
    // Close the file by dropping it
    drop(file);

    // Unlink (delete) the file
    std::fs::remove_file(file_path)?;

    Ok(())
}