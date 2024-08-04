use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::thread::sleep;
use std::time::Duration;
use memmap::MmapMut;

fn main() {
    // Create and open a file
    let file_path = "temp";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664) // Set permission
        .open(file_path)
        .expect("Unable to open file");

    // Set the size of the file (truncate)
    file.set_len(4096).expect("Unable to set file size");

    // Map the file into memory
    let mut mmap = unsafe { MmapMut::map_mut(&file).expect("Unable to map file") };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = b"Test message";
        let offset = 1024;

        // Write message to the offset
        let slice = &mut mmap[offset..offset + message.len()];
        slice.copy_from_slice(message);
        
        // Print the written message
        println!("Message written to the shared memory: {}", std::str::from_utf8(slice).unwrap());
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(&mmap[offset..offset + message.len()]).unwrap());
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // The MmapMut will be dropped here, automatically unmapping the memory
    drop(mmap); // Explicit drop (optional, as it's dropped at the end of the scope)

    // Delete the file
    std::fs::remove_file(file_path).expect("Unable to delete file");
}