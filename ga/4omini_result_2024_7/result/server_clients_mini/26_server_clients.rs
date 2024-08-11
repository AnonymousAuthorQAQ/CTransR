use std::fs::{File, remove_file};
use std::io::{Seek, SeekFrom, Write, Read};
use std::os::unix::io::AsRawFd;
use std::thread::sleep;
use std::time::Duration;
use memmap2::{MmapMut};

fn main() {
    // Create and open a file
    let file_path = "temp";
    let file = File::create(file_path).expect("Failed to create file");

    // Extend the file size to 4096 bytes
    file.set_len(4096).expect("Failed to set file length");

    // Create a writable memory map
    let mut mmap = unsafe { MmapMut::map_mut(&file).expect("Failed to map memory") };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let msg = "Test message";
        let offset = 1024; // Writing at offset 1024
        mmap[offset..offset + msg.len()].copy_from_slice(msg.as_bytes());
        println!("Message written to the shared memory: {}", msg);

        sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        let read_msg = std::str::from_utf8(&mmap[offset..offset + msg.len()]).unwrap();
        println!("Read from shared memory: {}", read_msg);

        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // The memory map is dropped here and automatically flushed to the file
    drop(mmap);

    // Remove the file
    remove_file(file_path).expect("Failed to remove file");
}