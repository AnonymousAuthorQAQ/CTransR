use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Create and open a file
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")?;
    
    // Set the size of the file
    file.set_len(4096)?;
    
    // Map the file into memory
    let mapping = unsafe { memmap::Mmap::map(&file)? };
    
    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut mapping[1024..1024 + "Test message".len()];
        p.copy_from_slice("Test message".as_bytes());
        println!("Message written to the shared memory: {}", String::from_utf8_lossy(p));
        std::thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", String::from_utf8_lossy(&mapping[1024..1024 + "Test message".len()]));
        std::thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    drop(mapping);

    file.sync_all()?;

    std::fs::remove_file("temp")?;

    Ok(())
}