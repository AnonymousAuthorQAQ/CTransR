use std::ffi::CString;
use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use memmap2::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("temp")?;
    
    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap = MmapOptions::new().len(4096).map_mut(&file)?;

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").expect("CString::new failed");
        let offset = 1024;
        
        unsafe {
            let dest = mmap.as_mut_ptr().add(offset);
            ptr::copy_nonoverlapping(message.as_ptr(), dest, message.as_bytes().len());
            *dest.add(message.as_bytes().len()) = 0; // null-terminate the string
        }

        // Print message written
        println!("Message written to the shared memory: {}", unsafe {
            CString::from_raw(mmap.as_mut_ptr().add(offset) as *mut _)
                .to_string_lossy()
                .into_owned()
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            CString::from_raw(mmap.as_mut_ptr().add(offset) as *mut _)
                .to_string_lossy()
                .into_owned()
        });

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close (Rust automatically handles cleanup)
    drop(mmap); // this will unmap the memory

    // Close the file (FAIR enough, Rust's Drop trait will handle this)
    // Remove the temporary file
    remove_file("temp")?;

    Ok(())
}