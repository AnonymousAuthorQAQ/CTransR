use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

fn main() {
    // Create and open a file
    let path = Path::new("temp");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
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
        panic!("mmap failed");
    }
    let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let msg = b"Test message\x00";
        let offset = 1024;
        slice[offset..offset + msg.len()].copy_from_slice(msg);

        println!("Message written to the shared memory: {}", str::from_utf8(&slice[offset..]).unwrap());
        std::thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!("Read from shared memory: {}", str::from_utf8(&slice[offset..]).unwrap());
        std::thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(addr, 4096);
    }
    file.sync_all().expect("Unable to sync file");

    std::fs::remove_file(path).expect("Unable to remove file");
}