use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

use libc::{c_void, mmap, munmap, open, O_RDWR, O_CREAT, ftruncate, MAP_SHARED, PROT_READ, PROT_WRITE, close, unlink};

const FILE_SIZE: usize = 4096;
const OFFSET: usize = 1024;

fn main() -> io::Result<()> {
    // Create and open a file
    let c_string = CString::new("temp").unwrap();
    let fd = unsafe { open(c_string.as_ptr(), O_RDWR | O_CREAT, 0o664) };
    if fd == -1 {
        eprintln!("Failed to open file");
        return Err(io::Error::last_os_error());
    }
    
    // Set the size of the file
    if unsafe { ftruncate(fd, FILE_SIZE as i64) } == -1 {
        eprintln!("Failed to set file size");
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            FILE_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        eprintln!("Failed to map memory");
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message\0"; // C-style string must be null-terminated
        unsafe {
            let p = ptr as *mut u8; // Cast to a mutable byte pointer
            ptr::copy_nonoverlapping(message.as_ptr(), p.add(OFFSET), message.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            slice::from_raw_parts((ptr as *const u8).add(OFFSET), message.len())
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output
        
        // Read from the same offset
        unsafe {
            let read_message = slice::from_raw_parts((ptr as *const u8).add(OFFSET), message.len());
            println!("Read from shared memory: {:?}", std::str::from_utf8(read_message).unwrap());
        }
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, FILE_SIZE) } == -1 {
        eprintln!("Failed to unmap memory");
        unsafe { close(fd) };
        return Err(io::Error::last_os_error());
    }

    unsafe { close(fd) };

    if unsafe { unlink(c_string.as_ptr()) } == -1 {
        eprintln!("Failed to unlink file");
        return Err(io::Error::last_os_error());
    }

    Ok(())
}