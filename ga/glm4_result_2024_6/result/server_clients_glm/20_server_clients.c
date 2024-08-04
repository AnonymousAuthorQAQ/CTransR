use std::io::{Error, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Error> {
    // Create and open a file
    let fd = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;
    
    // Set the size of the file
    fd.set_len(4096)?;

    // Map the file into memory
    let ptr = unsafe { libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd.as_raw_fd(), 0) };

    if ptr == libc::MAP_FAILED {
        return Err(Error::last_os_error());
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { slice::from_raw_parts_mut(ptr.offset(1024), 11) };
        p.copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", str::from_utf8(p)?);
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let p = unsafe { slice::from_raw_parts(ptr.offset(1024), 11) };
        println!("Read from shared memory: {}", str::from_utf8(p)?);
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        return Err(Error::last_os_error());
    }

    fd.close()?;

    if std::fs::remove_file("temp").is_err() {
        return Err(Error::last_os_error());
    }

    Ok(())
}