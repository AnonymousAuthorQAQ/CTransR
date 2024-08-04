use std::ffi::CString;
use std::os::unix::io::{FromRawFd, RawFd};
use std::{ptr, slice};
use std::fs::{OpenOptions};
use std::os::raw::c_int;
use libc::{c_void, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, c_char, ftruncate, lseek, close};
use libc::{mmap, munmap, unlink};

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");
    let fd = file.as_raw_fd() as c_int;

    // Set the size of the file
    unsafe {
        let res = ftruncate(fd, 4096);
        if res != 0 {
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
        let p = (ptr as *mut c_char).add(1024);
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            libc::strcpy(p, message.as_ptr());
            println!("Message written to the shared memory: {:?}", message);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));

        // Read from the same offset
        unsafe {
            println!("Read from shared memory: {:?}", CString::from_raw(slice::from_raw_parts(ptr as *const c_char, 4096)).to_string_lossy());
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) != 0 {
            panic!("Failed to unmap mapped memory");
        }
        close(fd);
    }

    // Cleanup - delete the file
    let file_path = CString::new("temp").expect("Failed to create file path CString");
    unsafe {
        unlink(file_path.as_ptr());
    }
}