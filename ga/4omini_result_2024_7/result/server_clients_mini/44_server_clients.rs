use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write};
use std::mem;
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;
use std::thread;
use std::time::Duration;
use memmap::{MmapMut, MmapOptions};

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
    let mut mmap: MmapMut = MmapOptions::new().len(4096).map_mut(&file)?;

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { mmap.as_mut_ptr().add(1024) as *mut u8 };
        let message = b"Test message\0";
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
        }
        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const _).to_string_lossy() });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const _).to_string_lossy() });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap is automatically handled when `mmap` goes out of scope
    // Close and remove the file
    remove_file("temp")?;
    
    Ok(())
}