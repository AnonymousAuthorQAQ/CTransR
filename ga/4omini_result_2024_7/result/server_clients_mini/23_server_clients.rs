use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::thread;
use std::time::Duration;
use mmap::{MemoryMap, MapOption};

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
    let mmap = MemoryMap::new(4096, &file, &[])?;
    let mut mmap_ptr = mmap.as_mut();

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").unwrap();
        let offset = 1024;
        let slice = &mut mmap_ptr[offset..offset + message.as_bytes().len() + 1]; // +1 for null terminator
        slice.copy_from_slice(&message.as_bytes_with_nul());

        // Output the message written to shared memory
        println!("Message written to the shared memory: {}", message.to_str().unwrap());

        // Sleep to simulate delay and observe the output
        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(&slice[..message.as_bytes().len()]).unwrap());

        // Sleep again to simulate delay
        thread::sleep(Duration::from_secs(1));
    }

    // Cleanup: the memory map will go out of scope and automatically be unmapped.
    // We also need to drop the file explicitly.
    drop(file);

    // Optionally remove the temporary file
    std::fs::remove_file("temp")?;

    Ok(())
}