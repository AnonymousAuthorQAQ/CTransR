use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::ptr;
use std::mem;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::raw::c_void;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    if let Err(err) = unsafe { libc::ftruncate(file.as_raw_fd(), 4096) } {
        panic!("Failed to truncate file: {}", err);
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("Failed to mmap file");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).offset(1024);
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", String::from_utf8_lossy(std::slice::from_raw_parts(p, 12)));

        // Read from the same offset
        println!("Read from shared memory: {}", String::from_utf8_lossy(std::slice::from_raw_parts((ptr as *mut u8).offset(1024), 12)));

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        stop += 1;
    }

    // Unmap the memory
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        panic!("Failed to munmap file");
    }

    drop(file);

    std::fs::remove_file("temp").expect("Failed to remove file");
}