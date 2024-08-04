use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let path = Path::new("temp");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

    // Map the file into memory
    let mapping = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if mapping == libc::MAP_FAILED {
        eprintln!("Failed to mmap file");
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { mapping.offset(1024) };
        unsafe {
            ptr::copy_nonoverlapping(b"Test message", p as *mut _, 12);
        }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const _).to_string_lossy()
        });
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const _).to_string_lossy()
        });
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(mapping, 4096) } == -1 {
        eprintln!("Failed to munmap file");
    }

    file.close().expect("Failed to close file");

    fs::remove_file(path).expect("Failed to remove file");
}