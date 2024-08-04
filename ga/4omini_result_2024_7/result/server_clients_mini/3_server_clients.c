use std::ffi::CString;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::mem;
use std::slice;
use std::time::Duration;
use std::thread;
use libc::{mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE, O_RDWR, O_CREAT};

const SIZE: usize = 4096;
const MESSAGE: &str = "Test message";

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;
    
    // Resize the file to SIZE
    fd.set_len(SIZE as u64)?;

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd.as_raw_fd(),
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
        let p = unsafe { slice::from_raw_parts_mut((ptr as *mut u8).add(1024), SIZE - 1024) };
        let message_cstr = CString::new(MESSAGE).unwrap();
        
        // Ensure we do not overflow the mapped memory
        if message_cstr.as_bytes().len() < p.len() {
            p[..message_cstr.as_bytes().len()].copy_from_slice(message_cstr.as_bytes());
            p[message_cstr.as_bytes().len()] = 0; // null-terminate the string
            println!("Message written to shared memory: {}", MESSAGE);
        }
        
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe {
            CStr::from_ptr((ptr as *const u8).add(1024) as *const i8)
                .to_string_lossy()
                .to_string()
        };

        println!("Read from shared memory: {}", read_message);
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap the memory
    if unsafe { munmap(ptr, SIZE) } == -1 {
        eprintln!("munmap failed");
        return Err(io::Error::last_os_error());
    }

    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}