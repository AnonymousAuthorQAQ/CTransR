use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

use libc::{c_int, c_void, ftruncate, mmap, munmap, close, unlink, O_RDWR, O_CREAT, PROT_READ, PROT_WRITE, MAP_SHARED, S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP};

fn main() {
    // Create and open a file
    let fd = unsafe {
        libc::open(
            b"temp\0".as_ptr() as *const libc::c_char,
            O_RDWR | O_CREAT,
            S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP,
        )
    };
    if fd == -1 {
        eprintln!("Error opening file: {}", io::Error::last_os_error());
        return;
    }

    // Set the size of the file
    if unsafe { ftruncate(fd, 4096) } == -1 {
        eprintln!("Error truncating file: {}", io::Error::last_os_error());
        unsafe { close(fd); }
        return;
    }

    // Map the file into memory
    let addr = unsafe { mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if addr == libc::MAP_FAILED {
        eprintln!("Error mapping file: {}", io::Error::last_os_error());
        unsafe { close(fd); }
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { addr.offset(1024) };
        let message = b"Test message\0";
        unsafe { ptr::copy(message.as_ptr(), p, message.len()); }
        println!("Message written to the shared memory: {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(p, message.len())) });

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(p, message.len())) });

        // Sleep to simulate delay
        std::thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(addr, 4096) } == -1 {
        eprintln!("Error unmapping file: {}", io::Error::last_os_error());
    }
    unsafe { close(fd); }

    if unsafe { unlink(b"temp\0".as_ptr() as *const libc::c_char) } == -1 {
        eprintln!("Error unlinking file: {}", io::Error::last_os_error());
    }
}