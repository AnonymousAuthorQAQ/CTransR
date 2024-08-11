use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // Set permissions to 664
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap: MmapMut = unsafe { MmapOptions::new().map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = b"Test message";
        let len = message.len();

        // Ensure the message fits
        if offset + len < mmap.len() {
            unsafe {
                ptr::copy_nonoverlapping(message.as_ptr(), mmap.as_mut_ptr().add(offset), len);
            }
            // Print the message written
            println!("Message written to the shared memory: {:?}", std::str::from_utf8(&mmap[offset..offset + len]).unwrap());
        }

        // Read from the same offset
        let read_message = &mmap[offset..offset + len];
        println!("Read from shared memory: {:?}", std::str::from_utf8(read_message).unwrap());

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay
    }

    // The memory-mapped file will be automatically un-mapped and closed when `mmap` goes out of scope.
    // Clean up the temporary file
    std::fs::remove_file("temp")?;

    Ok(())
}