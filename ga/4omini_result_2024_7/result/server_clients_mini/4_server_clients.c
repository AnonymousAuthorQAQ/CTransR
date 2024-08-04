use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::mem::MaybeUninit;
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap::{MmapMut, MmapOptions};

fn main() -> Result<(), Error> {
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
    let mut mmap = MmapOptions::new().len(4096).map_mut(&file)?;

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message\0"; // Append null terminator
        let msg_len = message.len();
        let offset = 1024;

        unsafe {
            // Copy the message into the memory-mapped area
            ptr::copy_nonoverlapping(
                message.as_ptr(),
                mmap.as_mut_ptr().add(offset),
                msg_len,
            );
        }

        // Read from the same offset
        let read_message = unsafe {
            let slice = std::slice::from_raw_parts(mmap.as_ptr().add(offset), msg_len);
            CString::from_vec_unchecked(slice.to_vec()).to_string_lossy()
        };
        
        println!("Message written to the shared memory: {}", read_message);
        
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay
    }

    // Unmap is handled automatically when `mmap` goes out of scope

    // Remove the file
    std::fs::remove_file(file_path)?;

    Ok(())
}