use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use libc::{c_void, close, ftruncate, mmap, munmap, open, unlink, PROT_READ, PROT_WRITE, MAP_SHARED, O_RDWR, O_CREAT};

fn main() {
    // Create and open a file
    let fd = unsafe { open(CString::new("temp").unwrap().as_ptr(), O_RDWR | O_CREAT, 0o664) };
    if fd == -1 {
        panic!("open failed");
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, 4096) } == -1 {
        panic!("ftruncate failed");
    }

    // Map the file into memory
    let ptr = unsafe { mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut c_char).offset(1024);
        unsafe {
            libc::strcpy(p, CString::new("Test message").unwrap().as_ptr());
            println!("Message written to the shared memory: {}", std::ffi::CStr::from_ptr(p).to_str().unwrap());
        }

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            println!("Read from shared memory: {}", std::ffi::CStr::from_ptr((ptr as *mut c_char).offset(1024)).to_str().unwrap());
        }

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        panic!("munmap failed");
    }

    unsafe {
        close(fd);
    }

    if unsafe { unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        panic!("unlink failed");
    }
}