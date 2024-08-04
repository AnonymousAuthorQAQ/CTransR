use std::ffi::CString;
use std::fs::{OpenOptions, remove_file};
use std::os::unix::fs::FileExt;
use std::ptr;
use std::time::Duration;
use std::thread;
use memmap::{MmapMut, Mmap};

// Import necessary system calls
use std::os::unix::io::AsRawFd;
use nix::fcntl::OFlag;
use nix::libc;
use nix::unistd::ftruncate;

fn main() -> std::io::Result<()> {
    // Create and open a file
    let file_path = "temp";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Set the size of the file
    ftruncate(file.as_raw_fd(), 4096).unwrap();

    // Map the file into memory
    let mut mmap_mut = unsafe {
        MmapMut::map_mut(&file).expect("failed to create mmap")
    };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let msg = CString::new("Test message").unwrap();
        let offset = 1024;
        let slice = &mut mmap_mut[offset..offset + msg.as_bytes().len()];
        slice.copy_from_slice(msg.as_bytes());

        // Null-terminate the string
        slice[msg.as_bytes().len()] = 0;

        // Read from the same offset
        let read_msg = unsafe {
            CStr::from_ptr(slice.as_ptr() as *const i8).to_string_lossy()
        };
        println!("Message written to the shared memory: {}", read_msg);
        
        // Sleep to simulate delay
        thread::sleep(Duration::from_secs(1));

        stop += 1;
    }

    // Clean up and unmap
    drop(mmap_mut); // This unmaps the memory safely

    // Close and delete the file
    remove_file(file_path)?;

    Ok(())
}