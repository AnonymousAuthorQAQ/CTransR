use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use std::fs::{OpenOptions, remove_file};
use libc::{c_void, c_char, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, munmap, ftruncate, close};
use nix::sys::mman::{mmap, MapFlags};

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
    if unsafe { ftruncate(fd, 4096) } == -1 {
        panic!("Failed to set file size");
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };

    if ptr == MAP_FAILED {
        panic!("Failed to map memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut c_char).offset(1024) };
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            libc::strcpy(p, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe {
            CString::from_raw(p).to_str().unwrap()
        });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            CString::from_raw((ptr as *mut c_char).offset(1024)).to_str().unwrap()
        });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        panic!("Failed to unmap memory");
    }

    unsafe {
        close(fd);
    }

    if let Err(err) = remove_file("temp") {
        panic!("Failed to remove file: {}", err);
    }
}