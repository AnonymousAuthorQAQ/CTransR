use std::io::{self, Write};
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Create and open a file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let mapping = unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );

        if ptr == libc::MAP_FAILED {
            Err(io::Error::last_os_error())?;
        }

        Box::from_raw(ptr)
    };

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { mapping.as_ptr().add(1024) };
        unsafe { std::ptr::write(p, b'T'); }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p).to_string_lossy()
        });

        // Sleep to simulate delay and observe the output
        std::thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p).to_string_lossy()
        });
        std::thread::sleep(Duration::from_secs(1));
    }

    // Unmap and close everything
    if unsafe { libc::munmap(mapping.as_ptr(), 4096) } == -1 {
        Err(io::Error::last_os_error())?;
    }

    // Delete the temp file
    std::fs::remove_file("temp")?;

    Ok(())
}