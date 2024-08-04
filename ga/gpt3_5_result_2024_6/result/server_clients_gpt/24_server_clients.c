use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::thread;
use std::time::Duration;

fn main() {
    // Open a file and set its size
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");
    
    file.set_len(4096)
        .expect("Failed to set file size");

    // Map the file into memory
    let ptr = unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("mmap failed");
        }
        ptr
    };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = ((ptr as usize) + 1024) as *mut libc::c_void;
        let message = CString::new("Test message").unwrap();
        unsafe {
            libc::strcpy(p as *mut libc::c_char, message.as_ptr());
            println!("Message written to the shared memory: {}", CStr::from_ptr(p as *const libc::c_char).to_str().unwrap());
        }

        // Read from the same offset
        let read_message;
        unsafe {
            read_message = CStr::from_ptr((ptr as *const libc::c_char).offset(1024)).to_str().unwrap();
            println!("Read from shared memory: {}", read_message);
        }

        stop += 1;
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, 4096) == -1 {
            panic!("munmap failed");
        }
    }

    drop(file);

    std::fs::remove_file("temp")
        .expect("Failed to remove file");
}