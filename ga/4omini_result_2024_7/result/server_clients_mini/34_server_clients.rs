use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use libc::{c_void, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};

const FILE_SIZE: usize = 4096;
const MESSAGE: &str = "Test message";

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
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
        eprintln!("mmap failed");
        return Err(io::Error::last_os_error());
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        unsafe {
            let message_cstring = CString::new(MESSAGE).unwrap();
            let message_ptr = message_cstring.as_ptr();
            let destination = (ptr as *mut u8).add(1024);
            ptr::copy_nonoverlapping(message_ptr, destination, MESSAGE.len());
            println!("Message written to the shared memory: {}", MESSAGE);
        }

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let read_ptr = (ptr as *const u8).add(1024);
            let read_message = std::str::from_utf8_unchecked(std::slice::from_raw_parts(read_ptr, MESSAGE.len()));
            println!("Read from shared memory: {}", read_message);
        }

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, FILE_SIZE) != 0 {
            eprintln!("munmap failed");
            return Err(io::Error::last_os_error());
        }
    }

    std::fs::remove_file("temp")?; // Remove the temporary file

    Ok(())
}