use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use memmap::{Mmap, Protection};

fn main() -> io::Result<()> {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")?;
    
    file.set_len(4096)?;

    // Map the file into memory
    let mmap = unsafe {
        Mmap::map_mut(&file)?
    };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let mut p = &mut mmap[1024..1024 + 13];
        p.write_all(b"Test message")?;
        println!("Message written to the shared memory: {}", std::str::from_utf8(&p).unwrap());

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let p = &mmap[1024..1024 + 13];
        println!("Read from shared memory: {}", std::str::from_utf8(p).unwrap());

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    Ok(())
}