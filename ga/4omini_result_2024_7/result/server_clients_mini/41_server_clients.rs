use std::fs::{File, remove_file};
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

extern crate libc;

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = File::create("temp")?;
    let fd_raw = fd.as_raw_fd();

    // Set the size of the file
    unsafe {
        if libc::ftruncate(fd_raw, 4096) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd_raw,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message\0"; // Ensure null-termination
        let p = (ptr as *mut u8).add(1024);
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
            println!("Message written to the shared memory: {:?}", std::ffi::CStr::from_ptr(p as *const i8));
            sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output
            
            // Read from the same offset
            println!("Read from shared memory: {:?}", std::ffi::CStr::from_ptr(p as *const i8));
            sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        }
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, 4096) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    // Close the file
    drop(fd); // Explicitly drop the file to close it

    // Remove the file
    remove_file("temp")?;

    Ok(())
}