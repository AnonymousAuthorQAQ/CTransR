use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::prelude::*;
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::thread;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o644)
        .open("temp")
        .unwrap();

    // Set the size of the file
    file.set_len(4096).unwrap();

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, file.as_raw_fd(), 0)
    };

    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).offset(1024) };
        let message = CString::new("Test message").unwrap();
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr() as *const i8);
        }
        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap() });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap() });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        panic!("munmap failed");
    }

    drop(file);

    std::fs::remove_file("temp").unwrap();
}