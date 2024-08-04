use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use libc::{c_void, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};

const FILE_SIZE: usize = 4096;
const MSG_OFFSET: usize = 1024;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // Set file permissions
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(FILE_SIZE as u64).expect("Failed to set file size");

    // Map the file into memory
    let fd = file.as_raw_fd();
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            FILE_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").expect("CString::new failed");
        unsafe {
            let p = (ptr as *mut u8).add(MSG_OFFSET);
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.as_bytes().len());
            *p.add(message.as_bytes().len()) = 0; // Null-terminate the string
            println!("Message written to the shared memory: {}", message.to_string_lossy());
        }
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let read_msg = std::ffi::CStr::from_ptr((ptr as *mut u8).add(MSG_OFFSET) as *const _);
            println!("Read from shared memory: {}", read_msg.to_string_lossy());
        }
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, FILE_SIZE) == -1 {
            panic!("munmap failed");
        }
    }

    std::fs::remove_file("temp").expect("Failed to delete temp file");
}