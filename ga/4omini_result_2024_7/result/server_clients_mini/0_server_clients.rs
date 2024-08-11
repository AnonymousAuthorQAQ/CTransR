use std::fs::{File, remove_file};
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::os::unix::io::AsRawFd;
use std::os::unix::fs::FileExt;
use std::time::Duration;
use std::thread;

const FILE_SIZE: usize = 4096;
const MESSAGE: &str = "Test message";

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = File::create("temp")?;
    fd.set_len(FILE_SIZE as u64)?;

    // Get raw file descriptor to use with `mmap`
    let raw_fd = fd.as_raw_fd();

    // Memory map the file
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            FILE_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            raw_fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe {
            ptr.add(1024 as usize) as *mut u8 // Move pointer to offset 1024
        };
        
        unsafe {
            // Use raw pointer to copy the string into the memory-mapped area
            let len = MESSAGE.len();
            ptr::copy_nonoverlapping(MESSAGE.as_ptr(), p, len);
            // Null-terminate the string
            *p.add(len) = 0;
        }

        println!("Message written to the shared memory: {:?}", unsafe { std::ffi::CStr::from_ptr(p as *const _).to_str().unwrap() });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output
        
        // Read from the same offset
        println!("Read from shared memory: {:?}", unsafe { std::ffi::CStr::from_ptr(p as *const _).to_str().unwrap() });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap the memory
    unsafe {
        libc::munmap(ptr, FILE_SIZE);
    }

    // Close the file (Rust will automatically clean up the file descriptor)
    remove_file("temp")?;

    Ok(())
}