use std::ffi::CString;
use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Duration;
use std::thread;
use memmap2::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;
    
    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap = MmapOptions::new().len(4096).map_mut(&file)?;

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut mmap[1024..];
        let message = "Test message";
        let bytes = message.as_bytes();

        // Ensure that the message fits in the buffer
        if bytes.len() < p.len() {
            // Copy the message to the mapped memory
            p[..bytes.len()].copy_from_slice(bytes);
            // Null-terminate the string
            p[bytes.len()] = 0;

            // Print message written to the shared memory
            println!("Message written to the shared memory: {}", message);

            // Sleep to simulate delay
            thread::sleep(Duration::from_secs(1));

            // Read from the same offset
            let read_message = unsafe { CStr::from_ptr(p.as_ptr() as *const i8) };
            println!("Read from shared memory: {:?}", read_message.to_str().unwrap());
            thread::sleep(Duration::from_secs(1));
        } else {
            println!("Message is too long to fit in the buffer.");
        }
    }

    // Unmap is handled automatically when the `MmapMut` goes out of scope
    drop(mmap);

    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}