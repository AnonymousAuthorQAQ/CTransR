use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write, Read};
use std::mem;
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;
use std::thread;
use std::time::Duration;

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

    // Memory mapping the file
    let mut buf: Vec<u8> = vec![0; 4096];
    let file_len = buf.len();
    
    {
        // Safety: We need to create a mutable reference to the memory
        let ptr = buf.as_mut_ptr();
        
        // Main loop
        for _ in 0..10 {
            // Write to a specific offset within the memory-mapped area
            let offset = 1024;
            let message = "Test message";
            unsafe {
                ptr::copy_nonoverlapping(
                    message.as_ptr(),
                    ptr.add(offset),
                    message.len()
                );
            }
            
            // Print the message written to the shared memory
            println!("Message written to the shared memory: {}", 
                     unsafe { std::str::from_utf8_unchecked(&buf[offset..offset + message.len()]) });
            
            // Simulate delay
            thread::sleep(Duration::from_secs(1));
            
            // Read from the same offset
            println!("Read from shared memory: {}", 
                     unsafe { std::str::from_utf8_unchecked(&buf[offset..offset + message.len()]) });
            
            // Simulate delay again
            thread::sleep(Duration::from_secs(1));
        }
    }
    
    // Cleanup: close the file and unlink it
    drop(file); // Explicitly drop the file handle
    
    remove_file(file_path)?;

    Ok(())
}