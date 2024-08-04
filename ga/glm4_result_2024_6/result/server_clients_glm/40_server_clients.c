use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

use libc::{c_void, ftruncate, mmap, munmap};

fn main() -> io::Result<()> {
    let path = Path::new("temp");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        ftruncate(fd, 4096);
    }

    // Map the file into memory
    let ptr = unsafe { mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };

    if ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let stop = 10;
    for _ in 0..stop {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { slice::from_raw_parts_mut(ptr.offset(1024), 1024) };
        let msg = b"Test message\x00";
        p.copy_from_slice(msg);

        println!("Message written to the shared memory: {}", str::from_utf8(p)?);

        std::thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        let p = unsafe { slice::from_raw_parts(ptr.offset(1024), 1024) };
        println!("Read from shared memory: {}", str::from_utf8(p)?);

        std::thread::sleep(Duration::from_secs(1));
    }

    // Unmap and close everything
    unsafe {
        munmap(ptr, 4096);
    }

    file.sync_all()?;

    std::fs::remove_file(path)?;

    Ok(())
}