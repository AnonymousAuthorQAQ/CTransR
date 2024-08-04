use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::mem;
use std::ptr;
use std::thread::sleep;
use std::time::Duration; 
use memmap::{MmapMut, Mmap};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;
    
    // Resize the file to 4096 bytes
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap_mut: MmapMut = unsafe {
        MmapMut::map_mut(&file)?
    };

    // Main loop
    let mut stop = 0;

    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message\0"; // Adding null character to make it a C-style string
        let offset = 1024;
        
        // Copy the message into the mmap area at the specified offset
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), mmap_mut.as_mut_ptr().add(offset), message.len());
        }

        // Print the written message
        println!("Message written to the shared memory: {}", String::from_utf8_lossy(&mmap_mut[offset..offset + message.len()]));
        
        // Sleep to simulate delay
        sleep(Duration::from_secs(1));

        // Read from the same offset, creating a string from it
        let read_message = &mmap_mut[offset..offset + message.len()];
        println!("Read from shared memory: {}", String::from_utf8_lossy(read_message));

        // Sleep again to simulate delay
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // The MmapMut will be automatically dropped here, ensuring the changes are saved and the mapping is released

    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}