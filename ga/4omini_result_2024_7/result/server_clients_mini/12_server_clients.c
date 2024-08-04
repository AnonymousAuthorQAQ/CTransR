use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use std::os::unix::fs::FileExt; // Required for seek and write_at functions

const FILE_SIZE: usize = 4096;
const MESSAGE: &str = "Test message";
const MESSAGE_OFFSET: usize = 1024;

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // Set the file permissions to 664
        .open("temp")?;

    // Set the size of the file
    file.set_len(FILE_SIZE as u64)?;

    // Memory map the file
    let fd = file.as_raw_fd();
    let mmap_ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            FILE_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if mmap_ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = mmap_ptr as *mut u8;
        let message_bytes = MESSAGE.as_bytes();
        
        unsafe {
            ptr::copy_nonoverlapping(message_bytes.as_ptr(), p.add(MESSAGE_OFFSET), message_bytes.len());
            // Null terminate the string
            *p.add(MESSAGE_OFFSET + message_bytes.len()) = 0; // Null character for string termination
        }

        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p.add(MESSAGE_OFFSET) as *const libc::c_char)
                .to_string_lossy()
        });

        thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p.add(MESSAGE_OFFSET) as *const libc::c_char)
                .to_string_lossy()
        });

        thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap the memory
    unsafe {
        libc::munmap(mmap_ptr, FILE_SIZE);
    }

    // Close the file
    drop(file);

    // Remove the temporary file
    std::fs::remove_file("temp")?;

    Ok(())
}