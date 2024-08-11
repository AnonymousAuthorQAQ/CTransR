use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;
use libc::{c_void, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, O_RDWR, O_CREAT, close, ftruncate, mmap, munmap};

fn main() {
    // Create and open a file
    let file = OpenOptions::new().read(true).write(true).create(true).open("temp").unwrap();
    let fd = file.into_raw_fd();

    // Set the size of the file
    unsafe {
        if ftruncate(fd, 4096) == -1 {
            panic!("ftruncate");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };
    if ptr == MAP_FAILED {
        panic!("mmap");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).add(1024);
        let msg = "Test message\0";
        unsafe {
            ptr::copy_nonoverlapping(msg.as_ptr(), p, msg.len());
            println!("Message written to the shared memory: {}", CString::from_raw(CString::from_ptr(p as *const u8).into_raw()).to_str().unwrap());
        }

        // Read from the same offset
        unsafe {
            let read_ptr = ptr.offset(1024);
            let read_slice = slice::from_raw_parts(read_ptr as *const u8, msg.len());
            println!("Read from shared memory: {}", CString::new(read_slice).expect("CString::new failed").to_str().unwrap());
        }

        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("munmap");
        }
        close(fd);
    }
}