use std::fs::{OpenOptions, File};
use std::io::{Error, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::time::Duration;
use std::thread;

const FILENAME: &str = "temp";
const FILE_SIZE: usize = 4096;

fn main() -> Result<(), Error> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILENAME)?;

    // Set the size of the file
    file.set_len(FILE_SIZE as u64)?;

    // Memory map the file
    use memmap2::MmapMut;
    let mmap = unsafe { MmapMut::map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message";
        let offset = 1024; // Offset in bytes

        let slice = unsafe {
            let ptr = mmap.as_mut_ptr().add(offset);
            slice::from_raw_parts_mut(ptr, FILE_SIZE - offset)
        };

        // Write the message to memory
        slice[..message.len()].copy_from_slice(message);
        println!(
            "Message written to the shared memory: {}",
            std::str::from_utf8(message).unwrap()
        );

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        let read_slice = unsafe {
            let ptr = mmap.as_ptr().add(offset);
            slice::from_raw_parts(ptr, FILE_SIZE - offset)
        };
        println!("Read from shared memory: {}", std::str::from_utf8(read_slice).unwrap());

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Ensure that changes are flushed to the file
    mmap.flush()?;

    // Close the file and clean up
    drop(mmap);
    drop(file);
    
    // Unlink the file (delete)
    std::fs::remove_file(FILENAME)?;

    Ok(())
}