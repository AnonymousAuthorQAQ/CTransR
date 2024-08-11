use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::thread;
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};
use libc::{MAP_SHARED, O_RDWR, PROT_READ, PROT_WRITE, ftruncate, mmap, munmap, unlink};

fn main() {
    let file = OpenOptions::new().read(true).write(true).create(true).open("temp").expect("Unable to create file");

    // Set the size of the file
    unsafe {
        let fd = file.as_raw_fd();
        if ftruncate(fd, 4096) == -1 {
            panic!("ftruncate failed");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, file.as_raw_fd(), 0)
    };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.as_bytes().len());
        }
        println!("Message written to the shared memory: {}", unsafe { CString::from_raw(p as *mut c_char).to_str().unwrap() });
        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        let read_message = unsafe { CString::from_raw((ptr as *mut u8).offset(1024) as *mut c_char).to_str().unwrap() };
        println!("Read from shared memory: {}", read_message);
        thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("munmap failed");
        }

        if unlink("temp\0".as_ptr() as *const c_char) == -1 {
            panic!("unlink failed");
        }
    }
}