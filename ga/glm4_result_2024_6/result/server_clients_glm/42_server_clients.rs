use std::io::{Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::ffi::CString;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let fd = unsafe { libc::open(CString::new("temp").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT, 0664) };
    if fd == -1 {
        eprintln!("Error opening file: {}", std::io::Error::last_os_error());
        return;
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, 4096) } == -1 {
        eprintln!("Error truncating file: {}", std::io::Error::last_os_error());
        unsafe { libc::close(fd); }
        return;
    }

    // Map the file into memory
    let ptr = unsafe { libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };
    if ptr == libc::MAP_FAILED {
        eprintln!("Error memory mapping file: {}", std::io::Error::last_os_error());
        unsafe { libc::close(fd); }
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { std::slice::from_raw_parts_mut(ptr.offset(1024 as isize), 1024) };
        p.copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p.as_ptr()).to_string_lossy() });

        // Sleep to simulate delay and observe the output
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let p = unsafe { std::slice::from_raw_parts(ptr.offset(1024 as isize), 1024) };
        println!("Read from shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p.as_ptr()).to_string_lossy() });

        // Sleep again to simulate delay
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        eprintln!("Error unmapping memory: {}", std::io::Error::last_os_error());
    }

    unsafe { libc::close(fd); }

    if std::fs::remove_file("temp").is_err() {
        eprintln!("Error deleting file: {}", std::io::Error::last_os_error());
    }
}