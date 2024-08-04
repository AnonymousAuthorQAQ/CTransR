use std::ptr;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::process::CommandExt;
use std::ffi::CString;
use std::time::Duration;
use std::cmp::Ordering;

fn main() {
    // Create and open a file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to create/open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file size");

    // Map the file into memory
    let map = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if map == libc::MAP_FAILED {
        panic!("Failed to memory map the file");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").unwrap();
        unsafe {
            ptr::copy(message.as_ptr(), map.offset(1024), message.as_bytes_with_nul().len());
        }

        println!("Message written to the shared memory: {}", unsafe {
            CString::from_raw(map.offset(1024)).into_string().unwrap()
        });

        std::thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            CString::from_raw(map.offset(1024)).into_string().unwrap()
        });

        std::thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(map, 4096) } == -1 {
        panic!("Failed to unmap the file");
    }
    
    // Close the file
    file.close().expect("Failed to close the file");

    // Remove the file
    if std::fs::remove_file("temp").is_err() {
        panic!("Failed to remove the file");
    }
}