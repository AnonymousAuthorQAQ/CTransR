use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::time::Duration;
use std::thread;
use libc::{c_void, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let fd = file.as_raw_fd();
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

    if ptr == libc::MAP_FAILED {
        eprintln!("mmap failed");
        std::process::exit(1);
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message";
        let p = unsafe { (ptr as *mut u8).add(1024) };
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.len());
            // Null-terminate the string
            *p.add(message.len()) = 0;
        }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const i8)
        }.to_string_lossy());

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const i8)
        }.to_string_lossy());
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap the memory
    if unsafe { munmap(ptr, 4096) } == -1 {
        eprintln!("munmap failed");
        std::process::exit(1);
    }

    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}