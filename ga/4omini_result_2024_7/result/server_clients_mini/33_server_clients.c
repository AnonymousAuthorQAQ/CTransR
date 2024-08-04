use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::time::Duration;
use std::thread;
use memmap::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // 0664 in octal
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap = MmapOptions::new().len(4096).map_mut(&file)?;

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = b"Test message\0"; // Null-terminated string
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), mmap.as_mut_ptr().add(offset), message.len());
        }
        println!("Message written to the shared memory: {:?}", &mmap[offset..offset + message.len() - 1]);

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe output

        // Read from the same offset
        let read_message = &mmap[offset..offset + message.len() - 1]; // Exclude null terminator when reading
        println!("Read from shared memory: {:?}", read_message);
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    drop(mmap); // Automatically unmaps when going out of scope

    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}