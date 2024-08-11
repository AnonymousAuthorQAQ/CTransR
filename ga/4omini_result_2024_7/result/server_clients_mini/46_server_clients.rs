use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file_path = "temp";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open(file_path)?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap: MmapMut = unsafe { MmapOptions::new().map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message";
        let offset = 1024;
        
        // Ensure that we don't exceed the bounds of the memory map
        if offset + message.len() < mmap.len() {
            // Copy the message to the memory-mapped area
            let p = &mut mmap[offset..offset + message.len()];
            p.copy_from_slice(message);
            
            // Print the message written to the shared memory
            println!("Message written to the shared memory: {:?}", std::str::from_utf8(p).unwrap());
        }

        // Read from the same offset
        let read_message = &mmap[offset..offset + message.len()];
        println!("Read from shared memory: {:?}", std::str::from_utf8(read_message).unwrap());

        // Sleep to simulate delay and observe the output
        thread::sleep(Duration::from_secs(1));
    }

    // The memory-mapped file will be automatically dropped when going out of scope,
    // and the file will be closed.

    // Remove the temporary file
    std::fs::remove_file(file_path)?;

    Ok(())
}