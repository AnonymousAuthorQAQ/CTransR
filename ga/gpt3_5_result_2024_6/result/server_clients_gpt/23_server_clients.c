use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::ffi::CString;
use std::mem;
use libc::{c_void, c_int, size_t, POSIX_MADV_SEQUENTIAL, truncate};
use libc::{mmap, munmap, close, unlink, MAP_SHARED, MAP_FAILED, PROT_READ, PROT_WRITE};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let fd: c_int = unsafe {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o664)
            .open("temp")
            .expect("Failed to open file")
            .as_raw_fd()
    };

    // Set the size of the file
    let _ = unsafe {
        truncate(CString::new("temp").unwrap().as_ptr(), 4096)
    };

    // Map the file into memory
    let ptr: *mut c_void = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    } as *mut c_void;
    if ptr == MAP_FAILED {
        panic!("Failed to map file to memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);
        let message = "Test message";
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
        }
        println!("Message written to the shared memory: {}", String::from_utf8_lossy(std::slice::from_raw_parts(p, message.len())));

        // Read from the same offset
        let read_message = String::from_utf8_lossy(std::slice::from_raw_parts((ptr as *mut u8).offset(1024), message.len()));
        println!("Read from shared memory: {}", read_message);

        sleep(Duration::from_secs(1));

        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        panic!("Failed to unmap memory");
    }

    if unsafe { close(fd) } == -1 {
        panic!("Failed to close file");
    }

    if unsafe { unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        panic!("Failed to unlink file");
    }
}