use std::fs;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Create and open a file
    let file = fs::File::create("temp")?;
    file.set_len(4096)?;

    // Memory-map the file
    let view = unsafe {
        std::memmap::Mmap::map(&file)?
    };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = &mut view[1024..];
        ptr::copy(b"Test message", p.as_mut_ptr(), 12);
        println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());
        std::thread::sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", str::from_utf8(&view[1024..]).unwrap());
        std::thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    drop(view);
    fs::remove_file("temp")?;

    Ok(())
}