use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::os::unix::prelude::*;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use mmap::{MemoryMap, MapOption};

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .mode(0o664) // Set file permissions
        .open("temp")?;

    // Set the size of the file
    fd.set_len(4096)?;

    // Map the file into memory
    let mut mmap = MemoryMap::new(4096, &fd, &MapOption::MapReadable | &MapOption::MapWritable | &MapOption::MapShared)?;

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message";
        let p = unsafe {
            let offset = 1024;
            let ptr = mmap.as_mut_ptr().add(offset);
            ptr::copy_nonoverlapping(message.as_ptr(), ptr, message.len());
            ptr
        };
        
        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy() });
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output
        
        // Read from the same offset
        let read_msg = unsafe { 
            std::ffi::CStr::from_ptr((mmap.as_mut_ptr() as usize + 1024) as *const i8).to_string_lossy() 
        };
        
        println!("Read from shared memory: {}", read_msg);
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    // The MemoryMap will be automatically dropped at the end of its scope
    drop(mmap);
    drop(fd);

    // Remove the temporary file
    std::fs::remove_file("temp")?;

    Ok(())
}