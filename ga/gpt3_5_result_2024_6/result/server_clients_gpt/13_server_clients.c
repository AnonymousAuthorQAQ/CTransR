use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .unwrap();
    
    // Set the size of the file
    file.set_len(4096).unwrap();

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr.offset(1024) as *mut i8) };
        let message = CString::new("Test message").unwrap();
        unsafe {
            libc::strcpy(p, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p)
        }.to_str().unwrap());
        
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe { std::ffi::CStr::from_ptr(ptr.offset(1024) as *const i8) };
        println!("Read from shared memory: {}", read_message.to_str().unwrap());
        
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap memory
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        panic!("munmap failed");
    }

    // Close the file
    drop(file);

    // Remove the temporary file
    std::fs::remove_file("temp").unwrap();
}