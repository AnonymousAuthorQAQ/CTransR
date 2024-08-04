use std::ffi::CString;
use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use nix::fcntl::{FcntlFlag, OFlag};
use nix::sys::mman::{mmap, Munmap, MapFlags, ProtFlags};
use nix::unistd::ftruncate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;

    // Set the size of the file
    ftruncate(file.as_raw_fd(), 4096)?;

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            4096,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    }?;

    // Main loop
    let mut stop = 0;
    let message = b"Test message\0"; // Null-terminated string
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        unsafe {
            // Copy the message into the mapped memory
            ptr::copy_nonoverlapping(message.as_ptr(), (ptr as *mut u8).add(1024), message.len());
        }
        println!("Message written to the shared memory: {:?}", unsafe {
            std::ffi::CStr::from_ptr((ptr as *mut u8).add(1024) as *const i8)
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe {
            std::ffi::CStr::from_ptr((ptr as *mut u8).add(1024) as *const i8)
        };
        println!("Read from shared memory: {:?}", read_message);
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        Munmap::unmap(ptr)?;
    }

    std::fs::remove_file("temp")?;

    Ok(())
}