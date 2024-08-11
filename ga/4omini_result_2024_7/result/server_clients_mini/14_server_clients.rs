use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::mem;
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
        .mode(0o664)
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mmap = unsafe { MmapOptions::new().len(4096).map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message";
        let message_bytes = message.as_bytes();
        let offset = 1024;
        
        // Writing directly with bounds checking
        let mmap_slice = unsafe {
            let slice = &mut mmap[offset..offset + message_bytes.len()];
            ptr::copy_nonoverlapping(message_bytes.as_ptr(), slice.as_mut_ptr(), message_bytes.len());
            slice
        };
        
        // Print the written message
        println!("Message written to the shared memory: {}", std::str::from_utf8(mmap_slice).unwrap());

        // Simulate delay
        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(&mmap[offset..offset + message_bytes.len()]).unwrap());
        
        // Simulate delay again
        thread::sleep(Duration::from_secs(1));
    }

    // The MmapMut instance will be automatically dropped
    // Unlink the file (delete it)
    std::fs::remove_file("temp")?;

    Ok(())
}