use std::fs::{File, remove_file};
use std::io::{self, Write};
use std::os::unix::fs::FileExt;
use std::ptr;
use std::slice;
use std::time::Duration;
use std::thread;
use memmap2::{MmapMut, MmapOptions}; // Make sure to add memmap2 in your Cargo.toml

fn main() -> io::Result<()> {
    // Create and open a file
    let file_path = "temp";
    let mut file = File::create(file_path)?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = b"Test message";
        let len = message.len();

        unsafe {
            let slice = slice::from_raw_parts_mut(mmap.as_mut_ptr().add(offset), 4096 - offset);
            ptr::copy_nonoverlapping(message.as_ptr(), slice.as_mut_ptr(), len);
            slice[len..len + 1].copy_from_slice(&[0]); // Null terminate the string
        }

        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(mmap.as_ptr().add(offset) as *const i8).to_string_lossy()
        });
        thread::sleep(Duration::new(1, 0)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(mmap.as_ptr().add(offset) as *const i8).to_string_lossy()
        });
        thread::sleep(Duration::new(1, 0)); // Sleep again to simulate delay

        stop += 1;
    }

    // No need to explicitly unmap; it will be un-mapped when going out of scope
    drop(mmap); // Ensure that mmap is dropped before moving on

    // Close the file (implicitly happens when file goes out of scope)
    remove_file(file_path)?;

    Ok(())
}