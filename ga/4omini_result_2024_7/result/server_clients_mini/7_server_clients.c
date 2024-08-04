use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap2::{MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    // Create and open a file
    let file_path = "temp";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mut mmap: MmapMut = unsafe { MmapOptions::new().map_mut(&file)? };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message";
        let offset = 1024;
        let len = message.len();

        unsafe {
            // Ensure that the memory is large enough to write the message
            ptr::copy_nonoverlapping(message.as_ptr(), mmap.as_mut_ptr().add(offset), len);
            // Null-terminate the string to avoid issues when reading it back
            *mmap.as_mut_ptr().add(offset + len) = 0;
        }

        // Print the written message
        println!("Message written to the shared memory: {:?}", &mmap[offset..offset + len]);
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe {
            std::ffi::CStr::from_ptr(mmap.as_ptr().add(offset) as *const i8)
        };
        println!("Read from shared memory: {:?}", read_message.to_str().unwrap());
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap is handled automatically when `mmap` goes out of scope
    drop(mmap); // Explicitly drop to unmap the memory

    // Remove the file
    std::fs::remove_file(file_path)?;

    Ok(())
}