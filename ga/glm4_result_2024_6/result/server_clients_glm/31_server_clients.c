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
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    // Set the size of the file
    if let Err(e) = fd.set_len(4096) {
        eprintln!("Error setting file length: {}", e);
        return;
    }

    // Map the file into memory
    let addr = unsafe {
        let addr = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        );
        if addr == libc::MAP_FAILED {
            eprintln!("Error mmaping file");
            return;
        }
        addr
    };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        p[1024..1024 + "Test message".len()].copy_from_slice("Test message".as_bytes());
        println!("Message written to the shared memory: {}", str::from_utf8(&p[1024..]).unwrap());
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", str::from_utf8(&p[1024..]).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(addr, 4096) } == -1 {
        eprintln!("Error munmaping file");
    }

    drop(fd);

    if let Err(e) = std::fs::remove_file("temp") {
        eprintln!("Error removing file: {}", e);
    }
}