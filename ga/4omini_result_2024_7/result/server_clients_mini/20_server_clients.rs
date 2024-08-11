use std::fs::{OpenOptions, remove_file};
use std::io::{Error, Write, Read};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::slice;
use std::thread;
use std::time::Duration;
use memmap::MmapMut;

fn main() -> Result<(), Error> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // Unix file permission
        .open("temp")?;

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

        // Ensure we have enough space for the message
        if bytes.len() < mmap.len() - offset {
            unsafe {
                let slice = slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut u8, mmap.len());
                let dest = &mut slice[offset..offset + bytes.len()];
                dest.copy_from_slice(bytes);
            }

            // Print the message written to shared memory
            println!("Message written to the shared memory: {}", message);

            // Read from the same offset
            let read_message = unsafe {
                slice::from_raw_parts(mmap.as_ptr().add(offset), bytes.len())
            };
            println!("Read from shared memory: {}", std::str::from_utf8(read_message).unwrap());

            // Simulate delay
            thread::sleep(Duration::from_secs(1));
        }

        stop += 1;
    }

    // Unmap is handled automatically by MmapMut when it goes out of scope
    drop(mmap);

    // Close the file
    drop(file); // Closed when `file` goes out of scope

    // Remove the temporary file
    remove_file("temp")?;

    Ok(())
}