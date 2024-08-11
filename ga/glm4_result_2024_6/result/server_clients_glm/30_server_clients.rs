use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use std::{mem, slice};

fn main() -> std::io::Result<()> {
    // Create and open a file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;
    file.set_len(4096)?;

    // Map the file into memory
    let map = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if map == libc::MAP_FAILED {
        Err(std::io::Error::last_os_error())
    } else {
        let slice = unsafe { slice::from_raw_parts_mut(map as *mut u8, 4096) };

        // Main loop
        let stop = 10;
        for _ in 0..stop {
            // Write to a specific offset within the memory-mapped area
            let msg = b"Test message";
            slice[1024..1024 + msg.len()].copy_from_slice(msg);
            println!("Message written to the shared memory: {}", unsafe {
                std::ffi::CStr::from_ptr(slice.as_ptr() as *const i8).to_string_lossy()
            });
            sleep(Duration::from_secs(1));

            // Read from the same offset
            println!("Read from shared memory: {}", unsafe {
                std::ffi::CStr::from_ptr(slice.as_ptr() as *const i8).to_string_lossy()
            });
            sleep(Duration::from_secs(1));
        }

        // Unmap and close everything
        if unsafe { libc::munmap(map, 4096) } == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            file.set_len(0)?;
            file.sync_all()?;
            std::fs::remove_file("temp")?;
            Ok(())
        }
    }
}