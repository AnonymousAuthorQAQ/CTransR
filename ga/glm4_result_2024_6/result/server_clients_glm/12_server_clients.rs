use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

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
        panic!("Failed to mmap");
    }

    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { addr.offset(1024) };
        let message = "Test message";
        unsafe {
            ptr::copy(message.as_ptr(), p, message.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(p, message.len()))
        });
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(p, message.len()))
        });
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(addr, 4096);
    }

    file.close().expect("Failed to close file");

    std::fs::remove_file("temp").expect("Failed to remove file");
}