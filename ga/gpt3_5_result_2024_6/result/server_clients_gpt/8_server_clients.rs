use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use libc::{c_void, c_int, c_char, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, MAP_ANONYMOUS, ftruncate, mmap, munmap, open, close, unlink, write};

fn main() {
    // Create and open a file
    let fd = unsafe { open(CString::new("temp").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o664) };
    if fd == -1 {
        panic!("Failed to open file");
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, 4096) } == -1 {
        panic!("Failed to truncate file");
    }

    // Map the file into memory
    let ptr = unsafe { mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if ptr == MAP_FAILED {
        panic!("Failed to map memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let p = (ptr as *mut c_char).offset(offset);
        let message = "Test message\0";
        unsafe {
            write(fd, message.as_ptr() as *const c_void, message.len());
        }
        println!("Message written to the shared memory: {}", unsafe { String::from_utf8_lossy(slice::from_raw_parts(p as *const u8, message.len())) });

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { String::from_utf8_lossy(slice::from_raw_parts(p as *const u8, message.len())) });

        stop += 1;
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        panic!("Failed to unmap memory");
    }

    unsafe {
        close(fd);
        unlink(CString::new("temp").unwrap().as_ptr());
    }
}