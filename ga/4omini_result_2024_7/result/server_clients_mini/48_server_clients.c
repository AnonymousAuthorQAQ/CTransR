use std::fs::{OpenOptions, remove_file};
use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;
use std::mem;
use std::time::Duration;
use std::thread::sleep;
use memmap::{MmapMut, Mmap};
use std::ptr;

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open("temp")?;

    // Set the size of the file
    fd.set_len(4096)?;

    // Map the file into memory
    let mut mmap = unsafe { MmapMut::map_mut(&fd)? };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = mmap.as_mut_ptr().add(1024);
        unsafe {
            ptr::write_bytes(p, 0, 12); // Clear the memory for the new string
            let msg = "Test message";
            let len = msg.len();
            ptr::copy_nonoverlapping(msg.as_ptr(), p, len);
        }

        let written_message = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(p, 12)) };
        println!("Message written to the shared memory: {}", written_message);
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(p, 12)) };
        println!("Read from shared memory: {}", read_message);
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    drop(mmap); // Drop the MmapMut for proper cleanup

    // Remove the file
    remove_file("temp")?;

    Ok(())
}