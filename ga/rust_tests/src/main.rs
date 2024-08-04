use std::fs::OpenOptions;
use std::io::Error;
use std::mem;
use std::ptr;
use std::slice;
use std::time::Duration;
use std::{thread, fs};
use libc::{c_void, mmap, munmap, shm_open, shm_unlink, O_RDWR, O_CREAT, PROT_READ, PROT_WRITE, MAP_SHARED};
use std::os::unix::io::AsRawFd;

const FILE_SIZE: usize = 4096;

fn main() -> Result<(), Error> {
    // Create and open a file
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("temp")?;

    // Set the size of the file
    file.set_len(FILE_SIZE as u64)?;

    // Map the file into memory
    let fd = file.as_raw_fd();
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
        return Err(Error::last_os_error());
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message\0"; // Null-terminated string
        unsafe {
            let p = ptr as *mut u8;
            let message_slice = slice::from_raw_parts(message.as_ptr(), message.len());
            ptr::copy_nonoverlapping(message_slice.as_ptr(), p.add(1024), message.len());
        }
        println!("Message written to the shared memory: Test message");

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let read_str = slice::from_raw_parts((ptr as *mut u8).add(1024), 14); // length of "Test message"
            let read_message = String::from_utf8_lossy(read_str);
            println!("Read from shared memory: {}", read_message);
        }
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap memory
    unsafe {
        if munmap(ptr, FILE_SIZE) != 0 {
            return Err(Error::last_os_error());
        }
    }

    // Clean up the file
    fs::remove_file("temp")?;

    Ok(())
}