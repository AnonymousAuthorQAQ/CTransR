use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let path = Path::new("temp");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("Unable to open file");

    // Set the size of the file
    file.set_len(4096).expect("Unable to set file length");

    // Map the file into memory
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
        eprintln!("mmap failed");
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { addr.offset(1024) };
        unsafe { str::from_utf8_unchecked(b"Test message").copy_to_slice(p as *mut [u8; 11]) };
        println!("Message written to the shared memory: {}", unsafe { str::from_utf8_unchecked(std::slice::from_raw_parts(p as *const u8, 11)) });
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { str::from_utf8_unchecked(std::slice::from_raw_parts(p as *const u8, 11)) });
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(addr, 4096) } == -1 {
        eprintln!("munmap failed");
    }

    file.sync_all().expect("Unable to sync file");
    std::fs::remove_file(path).expect("Unable to remove file");
}