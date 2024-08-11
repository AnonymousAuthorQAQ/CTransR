use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::IntoRawFd;
use std::fs::remove_file;
use libc::{mmap, MAP_FAILED, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FIXED, ftruncate, c_int, c_void};

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Failed to open file");
    
    let fd = file.into_raw_fd();

    // Set the size of the file
    let file_size: c_int = 4096;
    if unsafe { ftruncate(fd, file_size) } == -1 {
        panic!("Failed to set file size");
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), file_size as usize, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_FIXED, fd, 0)
    };
    if ptr == MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);
        let message = CString::new("Test message").expect("CString conversion failed");
        unsafe {
            libc::strcpy(p as *mut libc::c_char, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", message.to_str().unwrap());
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Read from the same offset
        let read_message = unsafe { std::ffi::CStr::from_ptr(p as *const libc::c_char) };
        println!("Read from shared memory: {}", read_message.to_str().unwrap());
        std::thread::sleep(std::time::Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, file_size as usize) } == -1 {
        panic!("munmap failed");
    }

    unsafe {
        libc::close(fd);
    }

    if remove_file("temp").is_err() {
        panic!("Failed to remove file");
    }
}