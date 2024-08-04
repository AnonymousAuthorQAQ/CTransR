use std::ffi::CString;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Duration;
use std::thread;
use std::mem;
use libc::{self, c_void};

fn main() {
    let filename = CString::new("temp").unwrap();

    // Create and open a file
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&filename)
        .expect("Failed to open file");

    // Set the size of the file
    let _ = fd.set_len(4096).expect("Failed to set file size");

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message\0";
        let p = unsafe {
            ptr.offset(1024) as *mut libc::c_char
        };
        
        unsafe {
            let message_ptr = CString::new(message).unwrap();
            ptr::copy_nonoverlapping(message_ptr.as_ptr(), p, message.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            CStr::from_ptr(p).to_string_lossy()
        });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            CStr::from_ptr(p).to_string_lossy()
        });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap the memory
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        panic!("munmap failed");
    }

    // Clean up - delete the file
    if std::fs::remove_file("temp").is_err() {
        panic!("Failed to unlink temp file");
    }
}