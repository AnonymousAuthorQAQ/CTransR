use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::ptr;
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
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

    // Map the file into memory
    let mapping = unsafe {
        std::slice::from_raw_parts_mut(
            libc::mmap(
                ptr::null_mut(),
                4096,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                0,
            ),
            4096,
        )
    };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut mapping[1024..];
        p.copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", String::from_utf8_lossy(p));

        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", String::from_utf8_lossy(&mapping[1024..]));
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(mapping.as_mut_ptr(), 4096).expect("Failed to unmap");
    }

    file.close().expect("Failed to close file");

    std::fs::remove_file(path).expect("Failed to remove file");
}