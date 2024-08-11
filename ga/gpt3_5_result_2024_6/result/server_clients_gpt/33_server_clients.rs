use std::ffi::CString;
use std::fs::{OpenOptions, File};
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use libc::{c_void, c_int, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, ftruncate, close, munmap, unlink};
use libc::{mmap, open, O_RDWR, O_CREAT};

fn main() {
    // Create and open a file
    let fd = unsafe {
        let filename = CString::new("temp").expect("CString conversion failed");
        open(filename.as_ptr(), O_RDWR | O_CREAT, 0o664)
    };
    if fd == -1 {
        eprintln!("Error opening file");
        return;
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, 4096) } == -1 {
        eprintln!("Error truncating file");
        return;
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    if ptr == MAP_FAILED {
        eprintln!("Error mapping file into memory");
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        unsafe {
            let p = ptr.offset(1024);
            let message = CString::new("Test message").expect("CString conversion failed");
            libc::strcpy(p as *mut i8, message.as_ptr());
            println!("Message written to the shared memory: {}", std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap());
        }
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let p = ptr.offset(1024);
            println!("Read from shared memory: {}", std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap());
        }
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, 4096) } == -1 {
        eprintln!("Error unmapping memory");
        return;
    }

    unsafe { close(fd) };

    if unsafe { unlink(CString::new("temp").expect("CString conversion failed").as_ptr()) } == -1 {
        eprintln!("Error unlinking file");
        return;
    }
}