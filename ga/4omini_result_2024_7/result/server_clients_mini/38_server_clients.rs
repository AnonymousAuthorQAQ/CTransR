use std::fs::OpenOptions;
use std::io::{self, Write, Seek, SeekFrom};
use std::os::unix::io::FromRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use std::mem;

// Include the necessary libraries for memory mapping
use memmap2::MmapMut;

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Memory map the file
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut mmap[1024..]; // Get the mmap slice starting at offset 1024
        let message = "Test message";
        p[..message.len()].copy_from_slice(message.as_bytes());
        p[message.len()] = 0; // Null-terminate the string

        // Print the message written to the shared memory
        println!("Message written to the shared memory: {}", unsafe {
            std::str::from_utf8_unchecked(&p[..message.len()])
        });
        
        // Sleep to simulate delay and observe the output
        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        let read_msg = &mmap[1024..];
        println!("Read from shared memory: {}", unsafe {
            std::str::from_utf8_unchecked(&read_msg[..message.len()])
        });

        // Sleep again to simulate delay
        thread::sleep(Duration::from_secs(1));
    }

    // Unmap and close everything does not need explicit unmanaged mapping in Rust
    // The MmapMut drops upon leaving the scope
    
    // Clean up the temporary file
    std::fs::remove_file("temp")?;

    Ok(())
}