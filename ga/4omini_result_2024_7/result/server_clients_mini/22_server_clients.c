use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::mem::MaybeUninit;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use std::os::unix::io::FromRawFd;
use libc::{mmap, munmap, O_RDWR, O_CREAT, MAP_SHARED, PROT_READ, PROT_WRITE};

const FILE_SIZE: usize = 4096;

fn main() -> io::Result<()> {
    // Create and open a file
    let path = CString::new("temp").unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open(&path)?;

    // Set the size of the file
    file.set_len(FILE_SIZE as u64)?;

    // Get the raw file descriptor
    let fd: RawFd = file.as_raw_fd();

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
        return Err(io::Error::last_os_error());
    }

    let stop: usize = 10;

    // Main loop
    for _ in 0..stop {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { ptr.offset(1024 as isize) };
        let message = "Test message".as_bytes();
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p as *mut u8, message.len());
            // Null-terminate the string
            *(p.add(message.len())) = 0;
        }

        // Print written message
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy()
        });

        sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const i8).to_string_lossy()
        });
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, FILE_SIZE) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    std::fs::remove_file("temp")?;
    Ok(())
}