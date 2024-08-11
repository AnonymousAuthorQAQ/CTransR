use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;
use std::thread;
use std::time::Duration;
use libc::{c_void, c_char, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, MAP_ANONYMOUS, msync, munmap};
use std::os::unix::io::AsRawFd;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o644)
        .open("temp")
        .expect("unable to open file");

    // Set the size of the file
    file.set_len(4096)
        .expect("unable to set file size");

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, file.as_raw_fd(), 0)
    };
    
    if ptr == MAP_FAILED {
        panic!("mapping failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        let offset: isize = 1024;
        let p = unsafe { (ptr as *mut i8).offset(offset) };
        let message = CString::new("Test message").expect("CString::new failed");
        unsafe {
            libc::strcpy(p, message.as_ptr() as *const c_char);
        }
        println!("Message written to the shared memory: {}", unsafe { String::from_raw_parts(p as *mut u8, 13, 13) });
        thread::sleep(Duration::from_secs(1));

        if let Ok(read_message) = unsafe { CString::from_raw(p as *mut c_char).into_string() } {
            println!("Read from shared memory: {}", read_message);
        }
        
        thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("munmap failed");
        }
    }

    // Unlink the file
    std::fs::remove_file("temp").expect("unable to unlink file");
}