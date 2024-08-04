use std::ffi::CString;
use std::os::raw::c_char;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

use libc::{c_void, ftruncate, mmap, munmap, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Failed to open file");

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        if ftruncate(fd, 4096) == -1 {
            panic!("Failed to truncate file");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            4096,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == MAP_FAILED {
        panic!("Failed to map memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe {
            (ptr as *mut c_char).offset(1024)
        };

        let message = CString::new("Test message").expect("CString conversion failed");
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p as *mut u8, message.as_bytes().len());
        }

        println!("Message written to the shared memory: {:?}", message);

        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        let read_ptr = unsafe {
            slice::from_raw_parts(ptr as *const c_char, 4096)
        };

        let read_message = CString::from_raw(read_ptr[1024..].as_ptr() as *mut c_char);
        println!("Read from shared memory: {:?}", read_message);

        thread::sleep(Duration::from_secs(1));

        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("Failed to unmap memory");
        }
    }

    drop(file);

    std::fs::remove_file("temp").expect("Failed to remove file");
}