use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::thread;
use std::time::Duration;

use libc::{mmap, munmap, open, O_RDWR, O_CREAT, unlink, PROT_READ, PROT_WRITE, MAP_SHARED, c_void, ftruncate};

const FILE_SIZE: usize = 4096;
const MESSAGE: &str = "Test message";

fn main() -> io::Result<()> {
    // Create and open a file
    let cstr = CString::new("temp").unwrap();
    let fd = unsafe { open(cstr.as_ptr(), O_RDWR | O_CREAT, 0o664) };
    
    if fd == -1 {
        eprintln!("open failed");
        std::process::exit(1);
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, FILE_SIZE as i64) } == -1 {
        eprintln!("ftruncate failed");
        std::process::exit(1);
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), FILE_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    
    if ptr == libc::MAP_FAILED {
        eprintln!("mmap failed");
        std::process::exit(1);
    }

    let mut stop = 0;

    // Main loop
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).add(1024) };
        unsafe {
            ptr::copy_nonoverlapping(MESSAGE.as_ptr(), p, MESSAGE.len());
            *p.add(MESSAGE.len()) = 0; // Null-terminate the string
        }

        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy() });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy() });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, FILE_SIZE) } == -1 {
        eprintln!("munmap failed");
        std::process::exit(1);
    }

    // Close the file descriptor
    let _ = unsafe { libc::close(fd) };

    // Unlink the file
    if unsafe { unlink(cstr.as_ptr()) } == -1 {
        eprintln!("unlink failed");
        std::process::exit(1);
    }

    Ok(())
}