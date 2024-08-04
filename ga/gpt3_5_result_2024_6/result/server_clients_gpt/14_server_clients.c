use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;

use libc::{c_void, c_char, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, ftruncate, mmap, munmap, close};

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Error opening file");

    let fd = file.as_raw_fd();

    // Set the size of the file
    match unsafe { ftruncate(fd, 4096) } {
        0 => {},
        _ => panic!("Error setting file size"),
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    if ptr == MAP_FAILED {
        panic!("Error mapping file into memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut c_char).offset(1024) };
        let message = CString::new("Test message").expect("CString::new failed");
        unsafe {
            libc::strcpy(p, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", message.to_str().expect("CString::to_str failed"));

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe { std::ffi::CStr::from_ptr((ptr as *mut c_char).offset(1024)) };
        println!("Read from shared memory: {}", read_message.to_str().expect("CStr::to_str failed"));

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        panic!("Error unmapping memory");
    }

    unsafe {
        close(fd);
    }

    if std::fs::remove_file("temp").is_err() {
        panic!("Error deleting file");
    }
}