use std::ffi::CString;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;

use libc::{c_void, ftruncate, mmap, munmap, MAP_FAILED, MAP_SHARED, O_CREAT, O_RDWR, PROT_READ, PROT_WRITE};

fn main() {
    // Create and open a file
    let fd = unsafe {
        libc::open(CString::new("temp").unwrap().as_ptr(), O_RDWR | O_CREAT, 0o664)
    };
    if fd == -1 {
        panic!("Failed to open file");
    }

    // Set the size of the file
    let result = unsafe { ftruncate(fd, 4096) };
    if result == -1 {
        panic!("Failed to set file size");
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    if ptr == MAP_FAILED {
        panic!("Failed to mmap file");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);
        unsafe {
            libc::strcpy(p, CString::new("Test message").unwrap().as_ptr());
            println!("Message written to the shared memory: {:?}", p);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));

        // Read from the same offset
        let read_data = unsafe {
            let data = (ptr as *const u8).offset(1024);
            std::ffi::CStr::from_ptr(data as *const i8).to_str().unwrap()
        };
        println!("Read from shared memory: {}", read_data);

        std::thread::sleep(std::time::Duration::from_secs(1));

        stop += 1;
    }

    // Unmap and close everything
    let result = unsafe { munmap(ptr, 4096) };
    if result == -1 {
        panic!("Failed to unmap file");
    }

    unsafe {
        libc::close(fd);
    }

    if unsafe { libc::unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        panic!("Failed to unlink file");
    }
}