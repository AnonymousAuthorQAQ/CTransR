use std::fs::{self, OpenOptions};
use std::io::{self, Write, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use libc::{c_void, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};

const FILE_NAME: &str = "temp";
const SIZE: usize = 4096;

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open(FILE_NAME)?;

    // Set the size of the file
    file.set_len(SIZE as u64)?;

    // Map the file into memory
    let fd = file.as_raw_fd();
    let ptr = unsafe {
        mmap(
            ptr::null_mut(), // Address (NULL means let the system choose)
            SIZE,            // Size
            PROT_READ | PROT_WRITE, // Desired memory protection
            MAP_SHARED,      // Shared mapping
            fd,              // File descriptor
            0,               // Offset
        ) as *mut c_void // Casting the pointer type
    };

    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as *mut u8).add(1024); // Offset by 1024
        let message = b"Test message\0";
        
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
        }

        // Print message written to the shared memory
        unsafe {
            let written_message = std::ffi::CStr::from_ptr(p as *const i8);
            println!("Message written to the shared memory: {:?}", written_message);
        }

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        unsafe {
            let read_message = std::ffi::CStr::from_ptr(p as *const i8);
            println!("Read from shared memory: {:?}", read_message);
        }
        
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap the memory
    if unsafe { munmap(ptr, SIZE) } == -1 {
        panic!("munmap failed");
    }

    // Close the file
    drop(file);

    // Remove the temporary file
    fs::remove_file(FILE_NAME)?;

    Ok(())
}