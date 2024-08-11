use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use libc::{c_void, c_char, O_RDWR, O_CREAT, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, ftruncate, mmap, munmap, close, unlink};

fn main() {
    // Create and open a file
    let fd = unsafe {
        libc::open(CString::new("temp").unwrap().as_ptr(), O_RDWR | O_CREAT, 0o664)
    };
    if fd == -1 {
        panic!("Failed to open file");
    }

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
        let p = unsafe {
            (ptr as *mut c_char).offset(1024)
        };
        unsafe {
            let message = "Test message\0";
            libc::strcpy(p, message.as_ptr() as *const c_char);
            println!("Message written to the shared memory: {}", CString::from_raw(p).to_str().unwrap());
        }
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_ptr = unsafe {
            slice::from_raw_parts(ptr as *const c_char, 4096)
        };
        let read_message = unsafe {
            CString::from_raw(read_ptr.as_ptr().offset(1024)).to_str().unwrap()
        };
        println!("Read from shared memory: {}", read_message);
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

    if unsafe { unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        panic!("Failed to unlink file");
    }
}