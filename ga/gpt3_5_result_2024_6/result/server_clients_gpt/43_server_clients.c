use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::{thread, time};
use std::fs::OpenOptions;
use libc::{c_void, c_char, off_t, PROT_READ, PROT_WRITE, MAP_SHARED, ftruncate, off_t, MAP_FAILED, c_int, close};

fn main() {
    // Create and open a file
    let file = OpenOptions::new().read(true).write(true).create(true).open("temp").unwrap();
    let file_fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        let size: off_t = 4096;
        if ftruncate(file_fd, size) == -1{
            panic!("ftruncate error");
        }
    }

    // Map the file into memory
    let ptr: *mut c_void = unsafe {
        libc::mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, file_fd, 0)
    };

    if ptr == MAP_FAILED {
        panic!("mmap error");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut c_char).offset(1024);
        let message = CString::new("Test message").unwrap();
        libc::strcpy(p, message.as_ptr());
        println!("Message written to the shared memory: {}", s as String);

        thread::sleep(time::Duration::from_secs(1));

        // Read from the same offset
        let read_data = std::slice::from_raw_parts((ptr as *mut c_char).offset(1024), 12);
        let read_str = std::str::from_utf8(read_data).unwrap();
        println!("Read from shared memory: {}", read_str);

        thread::sleep(time::Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, 4096) == -1 {
            panic!("munmap error");
        }

        close(file_fd as c_int);
    }

    //Delete the file
    std::fs::remove_file("temp").unwrap();
}