use std::ffi::CString;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

extern crate libc;
use libc::{c_void, c_int, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, MAP_ANON, MAP_FILE, MAP_FIXED, O_RDWR, O_CREAT, ftruncate, open, close, munmap};

const SIZE: usize = 4096;

fn main() {
    // Create and open a file
    let path = CString::new("temp").expect("CString conversion failed");
    let fd = unsafe { open(path.as_ptr(), O_RDWR | O_CREAT, 0o644) };
    if fd == -1 {
        panic!("Failed to open file");
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, SIZE as i64) } == -1 {
        panic!("Failed to truncate file");
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(ptr::null_mut(), SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    if ptr == MAP_FAILED {
        panic!("Failed to mmap file");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let p = unsafe { (ptr as *mut u8).offset(offset as isize) };
        let message = b"Test message\0";
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            String::from_utf8_lossy(slice::from_raw_parts(p, message.len()))
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            String::from_utf8_lossy(slice::from_raw_parts(p, message.len()))
        });

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, SIZE) } == -1 {
        panic!("Failed to unmap memory");
    }

    unsafe { close(fd) };

    let temp_path = CString::new("temp").expect("CString conversion failed");
    if unsafe { libc::unlink(temp_path.as_ptr()) } == -1 {
        panic!("Failed to unlink file");
    }
}