use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Create and open a file
    let file_path = Path::new("temp");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Set the size of the file
    file.set_len(4096)?;

    // Map the file into memory
    let map = unsafe {
        let ptr = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );

        if ptr == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }

        slice::from_raw_parts_mut(ptr as *mut u8, 4096)
    };

    // Main loop
    let stop = 10;
    for _ in 0..stop {
        // Write to a specific offset within the memory-mapped area
        let p = &mut map[1024..];
        let message = b"Test message";
        p.copy_from_slice(message);
        println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());

        // Read from the same offset
        println!("Read from shared memory: {}", str::from_utf8(&map[1024..]).unwrap());

        // Sleep to simulate delay and observe the output
        std::thread::sleep(Duration::from_secs(1));
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(map.as_mut_ptr() as *mut libc::c_void, 4096) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}