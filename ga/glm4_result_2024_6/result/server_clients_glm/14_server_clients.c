use std::io::prelude::*;
use std::os::unix::io::{FromRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;
use std::ffi::CString;

fn main() {
    // Create and open a file
    let path = CString::new("/tmp/temp").unwrap();
    let fd = unsafe {
        libc::open(path.as_ptr(), libc::O_RDWR | libc::O_CREAT, 0664)
    };
    if fd == -1 {
        eprintln!("Failed to open file: {}", std::io::Error::last_os_error());
        return;
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, 4096) } == -1 {
        eprintln!("Failed to ftruncate: {}", std::io::Error::last_os_error());
        unsafe { libc::close(fd) };
        return;
    }

    // Map the file into memory
    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    if addr == libc::MAP_FAILED {
        eprintln!("Failed to mmap: {}", std::io::Error::last_os_error());
        unsafe { libc::close(fd) };
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        let message = b"Test message";
        slice[1024..(1024 + message.len())].copy_from_slice(message);
        println!("Message written to the shared memory: {}", str::from_utf8(&slice[1024..(1024 + message.len())]).unwrap());

        // Sleep to simulate delay and observe the output
        std::thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", str::from_utf8(&slice[1024..(1024 + message.len())]).unwrap());

        // Sleep again to simulate delay
        std::thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(addr, 4096) } == -1 {
        eprintln!("Failed to munmap: {}", std::io::Error::last_os_error());
    }
    unsafe { libc::close(fd) };

    if let Err(e) = std::fs::remove_file(Path::new("/tmp/temp")) {
        eprintln!("Failed to remove file: {}", e);
    }
}