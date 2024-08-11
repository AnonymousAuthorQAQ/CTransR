use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::os::raw::c_void;
use std::ptr;
use std::thread;
use libc::{ftruncate, mmap, munmap, c_void, shm_unlink};

fn main() {
    // Create and open a file
    let fd = unsafe {
        let file = OpenOptions::new().read(true).write(true).create(true).open("temp").unwrap();
        file.into_raw_fd()
    };

    // Set the size of the file
    unsafe {
        if ftruncate(fd, 4096) == -1 {
            panic!("Error in ftruncate");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0)
    };
    if ptr == libc::MAP_FAILED {
        panic!("Error in mmap");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);
        let message = CString::new("Test message").unwrap();
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", std::str::from_utf8(std::slice::from_raw_parts(p, 12)).unwrap());
        thread::sleep(std::time::Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(std::slice::from_raw_parts(p, 12)).unwrap());
        thread::sleep(std::time::Duration::from_secs(1));

        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("Error in munmap");
        }
    }

    unsafe {
        let _ = File::from_raw_fd(fd);
        if shm_unlink("temp\0".as_ptr() as *const i8) == -1 {
            panic!("Error in shm_unlink");
        }
    }
}