use std::fs::{OpenOptions, remove_file};
use std::io::{Write, Read};
use std::mem;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use memmap::{MmapMut, MmapOptions};

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("temp")
        .expect("Failed to open the file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

    // Map the file into memory
    let mut mmap = unsafe { MmapOptions::new().len(4096).map_mut(&file) }
        .expect("Failed to create memory map");

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset = 1024;
        let message = b"Test message"; // Message to write
        let message_len = message.len();

        // Ensure we don't overflow the buffer
        if offset + message_len < mmap.len() {
            unsafe {
                ptr::copy_nonoverlapping(message.as_ptr(), mmap.as_mut_ptr().add(offset), message_len);
            }
            println!("Message written to the shared memory: {:?}", &mmap[offset..offset + message_len]);
        } else {
            println!("Message is too long to write to the offset.");
        }

        // Sleep to simulate delay and observe the output
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let read_message = &mmap[offset..offset + message_len];
        println!("Read from shared memory: {:?}", read_message);
        
        // Sleep again to simulate delay
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything (Mmap will drop automatically)
    drop(mmap); // Explicitly drop the mmap to ensure it's flushed
    drop(file); // Close the file

    // Unlink the file
    remove_file("temp").expect("Failed to unlink the file");

    // End of main function
}