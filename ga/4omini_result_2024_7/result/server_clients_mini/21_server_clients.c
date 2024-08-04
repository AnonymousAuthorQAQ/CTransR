use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use mmap::{MemoryMap, MapOption};

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
    let mmap = MemoryMap::new(4096, &[
        MapOption::MapReadable,
        MapOption::MapWritable,
        MapOption::MapShared,
        MapOption::MapFd(file.as_raw_fd()),
    ])?;

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { mmap.as_mut_slice() };
        let message = "Test message";
        let offset = 1024;
        let bytes = message.as_bytes();
        p[offset..offset + bytes.len()].copy_from_slice(bytes);
        
        // Print the written message
        println!("Message written to the shared memory: {}", std::str::from_utf8(&p[offset..offset + bytes.len()]).unwrap());
        sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(&p[offset..offset + bytes.len()]).unwrap());
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap is handled by dropping the MemoryMap object
    // Remove the file
    std::fs::remove_file(file_path)?;

    Ok(())
}