use std::io::{self, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::ffi::CString;
use std::ptr;
use std::time::Duration;
use std::thread::sleep;

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = unsafe { libc::open(CString::new("temp").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT, 0664) };
    if fd == -1 {
        eprintln!("Error opening file: {}", io::Error::last_os_error());
        return Err(io::Error::last_os_error());
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, 4096) } == -1 {
        eprintln!("Error setting file size: {}", io::Error::last_os_error());
        unsafe { libc::close(fd); }
        return Err(io::Error::last_os_error());
    }

    // Map the file into memory
    let ptr = unsafe { libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };
    if ptr == libc::MAP_FAILED {
        eprintln!("Error memory mapping file: {}", io::Error::last_os_error());
        unsafe { libc::close(fd); }
        return Err(io::Error::last_os_error());
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).offset(1024) as *mut libc::c_char };
        unsafe { libc::strcpy(p, CString::new("Test message").unwrap().as_ptr()); }
        println!("Message written to the shared memory: {}", unsafe { CString::from(p).into_string().unwrap() });
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { CString::from((ptr as *mut u8).offset(1024) as *mut libc::c_char).into_string().unwrap() });
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr as *mut libc::c_void, 4096) } == -1 {
        eprintln!("Error unmapping memory: {}", io::Error::last_os_error());
    }

    unsafe { libc::close(fd); }

    if unsafe { libc::unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        eprintln!("Error deleting file: {}", io::Error::last_os_error());
        return Err(io::Error::last_os_error());
    }

    Ok(())
}