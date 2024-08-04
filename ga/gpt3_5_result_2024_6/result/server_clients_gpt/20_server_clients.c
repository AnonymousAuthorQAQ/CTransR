use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    let file_len = 4096;

    // Set the size of the file
    file.set_len(file_len as u64)
        .expect("Failed to set file size");

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            file_len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("Failed to mmap");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).offset(1024) };
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.as_bytes().len());
        }
        println!("Message written to the shared memory: {:?}", message);

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe {
            let slice = slice::from_raw_parts(ptr as *const u8, file_len);
            std::str::from_utf8_unchecked(&slice[1024..1024 + message.as_bytes().len()])
        };
        println!("Read from shared memory: {:?}", read_message);

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay

        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(ptr, file_len);
    }

    drop(file);

    std::fs::remove_file("temp").expect("Failed to remove file");
}