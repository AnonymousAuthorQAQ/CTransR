use std::ffi::{CStr, CString};
use std::fs::OpenOptions;
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

extern crate libc;

fn main() {
    // Create and open the file
    let file_path = "temp";
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    // Set the size of the file
    let res = unsafe { libc::ftruncate(fd.as_raw_fd(), 4096) };
    if res == -1 {
        eprintln!("Error truncating file");
        return;
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        eprintln!("Error mapping file");
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").unwrap();
        let memory_offset = ptr as *mut i8; // Cast to mutable i8 pointer
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), memory_offset.offset(1024), message.to_bytes().len());
        }

        // Read from the same offset
        let read_data = unsafe { CStr::from_ptr(memory_offset.offset(1024)) };
        println!("Message written to the shared memory: {:?}", read_data.to_string_lossy());
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        println!("Read from shared memory: {:?}", read_data.to_string_lossy());
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        eprintln!("Error unmapping memory");
    }
    
    // Close the file (handled by Rust's Drop trait)

    // Delete the file
    if std::fs::remove_file(file_path).is_err() {
        eprintln!("Error removing file");
    }
}