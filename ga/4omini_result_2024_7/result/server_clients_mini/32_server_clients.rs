use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write, Read};
use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;
use memmap::MmapMut;

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
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = "Test message";
        let bytes = message.as_bytes();
        
        // Ensure the message fits into the allocated space
        if bytes.len() < mmap.len() - offset {
            mmap[offset..offset + bytes.len()].copy_from_slice(bytes);
            mmap.flush()?;
            println!("Message written to the shared memory: {}", message);
        } else {
            eprintln!("Message is too long to fit in the allocated space.");
        }

        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = std::str::from_utf8(&mmap[offset..offset + bytes.len()]).unwrap();
        println!("Read from shared memory: {}", read_message);
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything (handled automatically by Rust's ownership model)

    // Remove the temporary file
    remove_file(file_path)?;

    Ok(())
}