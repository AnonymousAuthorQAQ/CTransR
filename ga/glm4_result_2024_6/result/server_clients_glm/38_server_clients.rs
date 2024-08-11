use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

const FILENAME: &str = "temp";
const FILESIZE: usize = 4096;
const OFFSET: usize = 1024;
const MESSAGE: &str = "Test message";
const LOOP_COUNT: i32 = 10;
const SLEEP_DURATION: Duration = Duration::from_secs(1);

fn main() {
    // Create and open the file
    let file = std::fs::File::create(FILENAME).expect("Failed to create file");
    file.set_len(FILESIZE as u64).expect("Failed to set file length");

    // Memory map the file
    let raw_fd = file.as_raw_fd();
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            FILESIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            raw_fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        eprintln!("Failed to memory map file");
        std::process::exit(1);
    }

    // Main loop
    let mut stop = 0;
    while stop < LOOP_COUNT {
        // Write to the memory-mapped area
        unsafe {
            let dst = ptr.add(OFFSET);
            let message_bytes = MESSAGE.as_bytes();
            ptr::copy(message_bytes.as_ptr(), dst, message_bytes.len());
        }

        // Read from the memory-mapped area
        let read_bytes = unsafe {
            let src = ptr.add(OFFSET);
            let mut buffer = [0; MESSAGE.len()];
            ptr::copy(src, buffer.as_mut_ptr(), buffer.len());
            buffer
        };

        println!("Message written to the shared memory: {}", str::from_utf8(&read_bytes).unwrap());
        sleep(SLEEP_DURATION);
        println!("Read from shared memory: {}", str::from_utf8(&read_bytes).unwrap());
        sleep(SLEEP_DURATION);
        stop += 1;
    }

    // Unmap and close the file
    if unsafe { libc::munmap(ptr, FILESIZE) } == -1 {
        eprintln!("Failed to unmap file");
        std::process::exit(1);
    }

    drop(file);

    // Delete the temporary file
    if let Err(e) = std::fs::remove_file(FILENAME) {
        eprintln!("Failed to delete file: {}", e);
        std::process::exit(1);
    }
}