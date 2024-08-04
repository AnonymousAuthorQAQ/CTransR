use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Duration;
use std::thread;
use memmap2::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // equivalent to 0664 in C
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap: MmapMut = unsafe {
        MmapOptions::new()
            .len(4096)
            .map_mut(&file)?
    };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = "Test message".as_bytes();
        let len = message.len();

        // Copy message to the mapped memory, ensuring to handle the lengths correctly
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), mmap.as_mut_ptr().add(offset), len);
            mmap[len + offset - 1] = 0; // Null-terminate the string
        }

        // Convert to a Rust string for printing (since the Rust string handling is safer)
        let written_message = unsafe { std::str::from_utf8_unchecked(&mmap[offset..offset + len]) };
        println!("Message written to the shared memory: {}", written_message);
        
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe { std::str::from_utf8_unchecked(&mmap[offset..offset + len]) };
        println!("Read from shared memory: {}", read_message);
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Ensure the mapped memory is flushed (not strictly necessary for this example)
    mmap.flush()?;

    // Cleanup (The file will be deleted at the end of the program)
    drop(mmap); // Explicitly drop mmap before the file is closed

    std::fs::remove_file("temp")?; // Remove the temporary file
    Ok(())
}