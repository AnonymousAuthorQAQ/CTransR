use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::time::Duration;
use std::thread;
use libc::{c_void, mmap, munmap, open, ftruncate, O_RDWR, O_CREAT, MAP_SHARED, PROT_READ, PROT_WRITE};

const FILE_SIZE: usize = 4096;

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;

    // Set the size of the file
    let fd: RawFd = file.as_raw_fd();
    unsafe {
        if ftruncate(fd, FILE_SIZE as i64) == -1 {
            eprintln!("ftruncate failed");
            std::process::exit(1);
        }
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
    let stop = 10;
    for _ in 0..stop {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).add(1024);
        let message = CString::new("Test message").unwrap();
        unsafe {
            let message_slice = slice::from_raw_parts_mut(p, message.as_bytes().len());
            message_slice.copy_from_slice(message.as_bytes());
            println!("Message written to the shared memory: {}", message.to_string_lossy());
        }

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let read_message = CStr::from_ptr(p as *const c_char);
            println!("Read from shared memory: {}", read_message.to_string_lossy());
        }
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, FILE_SIZE) == -1 {
            eprintln!("munmap failed");
            std::process::exit(1);
        }
    }

    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}