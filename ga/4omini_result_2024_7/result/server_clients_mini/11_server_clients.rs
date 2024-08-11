use std::fs::{OpenOptions, File};
use tempfile::NamedTempFile;
use memmap2::{MmapMut, MmapOptions};
use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Create a temporary file
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path();

    // Open the file for reading and writing
    let file = OpenOptions::new().read(true).write(true).open(&file_path)?;

    // Set the size of the file to 4096 bytes
    file.set_len(4096)?;

    // Memory map the file
    let mut mmap: MmapMut = unsafe { MmapOptions::new().len(4096).map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut mmap[1024..]; // Slice the mapped memory
        p[..13].copy_from_slice(b"Test message\0"); // Copy the message
        println!("Message written to the shared memory: {}", std::str::from_utf8(&p[..13]).unwrap().trim_end_matches('\0'));
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(&mmap[1024..1037]).unwrap().trim_end_matches('\0'));
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // The temporary file will be automatically deleted when the `NamedTempFile` goes out of scope

    Ok(())
}