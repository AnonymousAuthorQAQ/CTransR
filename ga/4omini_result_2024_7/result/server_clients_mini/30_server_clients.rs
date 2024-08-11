use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use libc::{c_void, mmap, munmap, shm_open, ftruncate, MAP_SHARED, PROT_READ, PROT_WRITE, O_RDWR, O_CREAT, S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP};

const SHM_SIZE: usize = 4096;

fn main() {
    // Create and open a file
    let file_path = CString::new("temp").unwrap();
    let fd = unsafe {
        shm_open(file_path.as_ptr(), O_RDWR | O_CREAT, S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP)
    };
    
    if fd == -1 {
        eprintln!("Error opening file");
        return;
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, SHM_SIZE as i64) } == -1 {
        eprintln!("Error truncating file");
        return;
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), SHM_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    
    if ptr == libc::MAP_FAILED {
        eprintln!("Error mapping memory");
        return;
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).add(1024);
        let message = "Test message\0"; // Null-terminated string
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
        }

        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy() });
        sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy() });
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap the memory
    if unsafe { munmap(ptr, SHM_SIZE) } == -1 {
        eprintln!("Error unmapping memory");
        return;
    }

    // Close the file descriptor
    let _ = unsafe { File::from_raw_fd(fd) }; // Automatically closes the fd when going out of scope

    // Remove the file
    if std::fs::remove_file("temp").is_err() {
        eprintln!("Error removing file");
    }
}