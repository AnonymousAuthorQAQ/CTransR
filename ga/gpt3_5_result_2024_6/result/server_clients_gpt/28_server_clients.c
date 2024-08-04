use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use libc::{c_void, c_char, O_RDWR, O_CREAT, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, MAP_SHARED, MAP_FAILED, ftruncate, close, unlink, mmap, munmap};

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        if ftruncate(fd, 4096) != 0 {
            panic!("Failed to truncate file");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };

    if ptr == MAP_FAILED {
        panic!("Failed to map file into memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut c_char).offset(1024);
        let message = "Test message";
        unsafe {
            let c_str = CString::new(message).expect("Failed to convert message to CString");
            libc::strcpy(p, c_str.as_ptr());
        }
        println!("Message written to the shared memory: {}", message);

        std::thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_ptr = ptr as *const c_char;
        let read_slice = unsafe { slice::from_raw_parts(read_ptr.offset(1024), message.len()) };
        let read_str = std::str::from_utf8(read_slice).expect("Failed to read from memory");
        println!("Read from shared memory: {}", read_str);

        std::thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) != 0 {
            panic!("Failed to unmap memory");
        }
    }

    close(fd);

    if unlink("temp") != 0 {
        panic!("Failed to unlink file");
    }
}