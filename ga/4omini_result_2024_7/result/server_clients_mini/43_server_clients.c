use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Duration;
use std::thread;

use libc::{c_void, mmap, munmap, open, O_RDWR, O_CREAT, ftruncate, PROT_READ, PROT_WRITE, MAP_SHARED};

const FILE_SIZE: usize = 4096;
const TEMP_FILE: &str = "temp";

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TEMP_FILE)?;

    // Set the size of the file
    let fd = file.as_raw_fd();
    if unsafe { ftruncate(fd, FILE_SIZE as i64) } == -1 {
        eprintln!("ftruncate failed");
        std::process::exit(1);
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            FILE_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        eprintln!("mmap failed");
        std::process::exit(1);
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").unwrap();
        unsafe {
            let p = ptr as *mut i8;
            ptr::copy_nonoverlapping(message.as_ptr(), p.add(1024), message.to_bytes().len());
            println!("Message written to the shared memory: {}", message.to_string_lossy());
        }
        
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let read_msg = std::ffi::CStr::from_ptr((ptr as *mut i8).add(1024));
            println!("Read from shared memory: {}", read_msg.to_string_lossy());
        }
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, FILE_SIZE) } == -1 {
        eprintln!("munmap failed");
        std::process::exit(1);
    }

    // Delete the temporary file
    std::fs::remove_file(TEMP_FILE)?;

    Ok(())
}