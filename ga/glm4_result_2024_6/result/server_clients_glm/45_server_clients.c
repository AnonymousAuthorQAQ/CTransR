use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let fd = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
    {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    // Set the size of the file
    if let Err(e) = fd.set_len(4096) {
        eprintln!("Failed to set file length: {}", e);
        return;
    }

    // Map the file into memory
    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        )
    };
    if addr == libc::MAP_FAILED {
        eprintln!("Failed to mmap file");
        return;
    }
    let mut addr = addr as *mut libc::c_void;

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = addr.offset(1024);
        unsafe {
            std::ptr::copy_nonoverlapping(b"Test message", p as *mut u8, 11);
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                p as *const u8,
                11,
            ))
        });
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                p as *const u8,
                11,
            ))
        });
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(addr, 4096) } == -1 {
        eprintln!("Failed to munmap file");
    }

    if let Err(e) = fd.sync_all() {
        eprintln!("Failed to sync file: {}", e);
    }

    if let Err(e) = std::fs::remove_file("temp") {
        eprintln!("Failed to remove file: {}", e);
    }
}