use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::mem::size_of;
use std::ptr;
use std::slice;
use libc::{mmap, munmap, c_void, MAP_SHARED, PROT_READ, PROT_WRITE, open, close, unlink, ftruncate};

fn main() {
    // Create and open a file
    let fd = unsafe { open(CString::new("temp").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o664) };
    if fd == -1 {
        panic!("Failed to open the file");
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, 4096) } == -1 {
        panic!("Failed to set file size");
    }
    
    // Map the file into memory
    let ptr = unsafe { mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if ptr == libc::MAP_FAILED {
        panic!("Failed to map file into memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut c_char).offset(1024) };
        unsafe {
            ptr::copy_nonoverlapping(CString::new("Test message").unwrap().into_raw().as_ptr(), p as *mut u8, 12);
        }
        println!("Message written to the shared memory: {}", unsafe { CString::from_raw(p).to_str().unwrap() });
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { CString::from_raw(p).to_str().unwrap() });
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        panic!("Failed to unmap memory");
    }
    
    unsafe { close(fd) };

    if unsafe { unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        panic!("Failed to unlink the file");
    }
}