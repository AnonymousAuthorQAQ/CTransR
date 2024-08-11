use std::fs;
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Create and open a file
    let mut file = fs::File::create("temp")?;
    file.set_len(4096)?;

    // Memory map the file
    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if addr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }
    let mut addr = addr as *mut libc::c_void;

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = addr.offset(1024);
        let message = "Test message";
        unsafe {
            std::ptr::copy(message.as_ptr(), p, message.len());
            std::ptr::write(p.offset(message.len() as isize), 0);
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(p as *const u8, message.len()))
        });

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(p as *const u8, message.len()))
        });

        std::thread::sleep(Duration::from_secs(1));
    }

    // Unmap and close everything
    if unsafe { libc::munmap(addr, 4096) } == -1 {
        return Err(io::Error::last_os_error());
    }

    fs::remove_file("temp")?;

    Ok(())
}