use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
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
        .mode(0o644)
        .open("temp")
        .unwrap();

    let file_size: u64 = 4096;

    // Set the size of the file
    file.set_len(file_size).unwrap();

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            file_size as libc::size_t,
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
        let offset: usize = 1024;

        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).offset(offset as isize) };
        let message = CString::new("Test message").unwrap();
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe {
            CStr::from_ptr(p as *mut i8).to_str().unwrap()
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!(
            "Read from shared memory: {}",
            unsafe { CStr::from_ptr(p as *mut i8).to_str().unwrap() }
        );

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(ptr, file_size as libc::size_t);
    }

    // Close the file
    let _ = unsafe { File::from_raw_fd(file.into_raw_fd()) };

    // Delete the file
    std::fs::remove_file("temp").unwrap();
}