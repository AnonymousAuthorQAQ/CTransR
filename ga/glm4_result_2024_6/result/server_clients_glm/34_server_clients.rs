use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

const FILENAME: &str = "temp";
const FILESIZE: usize = 4096;
const MESSAGE: &str = "Test message";
const OFFSET: usize = 1024;
const ITERATIONS: usize = 10;

fn main() {
    // Open or create the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILENAME)
        .expect("Failed to open file");

    // Set the file size
    file.set_len(FILESIZE as u64).expect("Failed to set file size");

    // Memory map the file
    let mmap = unsafe {
        let addr = libc::mmap(
            ptr::null_mut(),
            FILESIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );

        if addr == libc::MAP_FAILED {
            panic!("Failed to memory map file");
        }

        addr
    };

    // Main loop
    for _ in 0..ITERATIONS {
        // Write to the memory-mapped area
        let message_bytes = MESSAGE.as_bytes();
        let dst = unsafe { slice::from_raw_parts_mut(mmap.offset(OFFSET as isize), message_bytes.len()) };
        dst.copy_from_slice(message_bytes);

        println!("Message written to the shared memory: {}", str::from_utf8(dst).unwrap());

        // Read from the memory-mapped area
        let src = unsafe { slice::from_raw_parts(mmap.offset(OFFSET as isize), message_bytes.len()) };
        println!("Read from shared memory: {}", str::from_utf8(src).unwrap());

        // Sleep for 1 second
        sleep(Duration::from_secs(1));
    }

    // Unmap the memory
    let res = unsafe { libc::munmap(mmap as *mut libc::c_void, FILESIZE) };
    if res != 0 {
        panic!("Failed to unmap memory");
    }

    // Close the file
    drop(file);

    // Remove the temporary file
    if let Err(e) = std::fs::remove_file(FILENAME) {
        eprintln!("Failed to remove temporary file: {}", e);
    }
}