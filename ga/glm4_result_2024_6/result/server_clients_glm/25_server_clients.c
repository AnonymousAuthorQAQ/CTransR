use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

use libc::{c_void, ftruncate, mmap, munmap, unlink};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;
    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe { ftruncate(fd, 4096)? };

    // Map the file into memory
    let addr = unsafe { mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };
    if addr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = addr as *mut u8;
        let slice = unsafe { slice::from_raw_parts_mut(p.offset(1024), 11) };
        slice.copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", str::from_utf8(slice).unwrap());
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let slice = unsafe { slice::from_raw_parts(p.offset(1024), 11) };
        println!("Read from shared memory: {}", str::from_utf8(slice).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe { munmap(addr, 4096)? };

    file.sync_all()?;

    let _ = unlink("temp");

    Ok(())
}