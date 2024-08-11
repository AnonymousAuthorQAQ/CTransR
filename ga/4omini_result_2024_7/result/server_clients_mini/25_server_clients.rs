use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write, Seek, SeekFrom};
use std::mem::size_of;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::time::Duration;
use std::thread;
use memmap2::{MmapMut, MmapOptions};

const FILE_SIZE: usize = 4096;
const MESSAGE: &str = "Test message";
const OFFSET: usize = 1024;

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;
    
    // Set the size of the file
    file.set_len(FILE_SIZE as u64)?;

    // Map the file into memory
    let mut mmap = unsafe {
        MmapOptions::new()
            .len(FILE_SIZE)
            .map_mut(&file)?
    };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message_bytes = MESSAGE.as_bytes();
        let offset = OFFSET;
        
        // Ensure we do not exceed the boundaries of the mmap
        if offset + message_bytes.len() < mmap.len() {
            unsafe {
                ptr::copy_nonoverlapping(message_bytes.as_ptr(), mmap.as_mut_ptr().add(offset), message_bytes.len());
            }
            println!("Message written to the shared memory: {}", MESSAGE);
        }

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe { std::str::from_utf8_unchecked(&mmap[offset..offset + message_bytes.len()]) };
        println!("Read from shared memory: {}", read_message);
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything (Rust automatically handles cleanup when `mmap` goes out of scope)
    
    // Remove the file
    remove_file("temp")?;

    Ok(())
}