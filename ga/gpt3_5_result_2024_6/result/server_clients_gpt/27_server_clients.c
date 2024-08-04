use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::raw::c_void;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Cannot open file");

    let size = 4096;
    
    // Set the size of the file
    file.set_len(size as u64)
        .expect("Cannot set file size");

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            size,
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
        let p = (ptr as *mut i8).offset(1024);
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            libc::strcpy(p, message.as_ptr());
        }
        println!("Message written to the shared memory: {:?}", &*(p as *const i8));
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {:?}", &*((ptr as *mut i8).offset(1024)));
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, size) == -1 {
            panic!("munmap failed");
        }
    }
}