use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{Result, Error};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;

fn main() -> Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;
    
    let fd = file.as_raw_fd();

    // Set the size of the file
    if let Err(err) = unsafe { libc::ftruncate(fd, 4096) } {
        return Err(Error::from_raw_os_error(err));
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        return Err(Error::last_os_error());
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).add(1024) };
        unsafe {
            let msg = CString::new("Test message").expect("CString::new failed");
            libc::strcpy(p as *mut libc::c_char, msg.as_ptr());
        }
        println!("Message written to the shared memory: {}", std::str::from_utf8(unsafe { std::slice::from_raw_parts(p, 13) }).unwrap());
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(unsafe { std::slice::from_raw_parts((ptr as *mut u8).add(1024), 13) }).unwrap());
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        return Err(Error::last_os_error());
    }

    drop(file);

    if let Err(err) = std::fs::remove_file("temp") {
        return Err(err);
    }

    Ok(())
}