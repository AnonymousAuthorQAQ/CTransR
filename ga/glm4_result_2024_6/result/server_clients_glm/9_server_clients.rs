use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};
use memmap::{Mmap, MmapMut, MmapOptions};

fn main() -> io::Result<()> {
    let path = Path::new("temp");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    file.set_len(4096)?;

    let mut mmap = MmapMut::map(&file)?; // Mutable mapping for read/write access

    let stop = 10;
    for _ in 0..stop {
        let msg = "Test message";
        let offset = 1024;
        // Write the message to the offset
        let dst_slice = &mut mmap[offset..offset + msg.len()];
        dst_slice.copy_from_slice(msg.as_bytes());
        println!("Message written to the shared memory: {}", msg);

        // Sleep to simulate delay
        std::thread::sleep(Duration::from_secs(1));

        // Read the message from the offset
        let read_slice = &mmap[offset..offset + msg.len()];
        let read_msg = std::str::from_utf8(read_slice).expect("Invalid UTF-8");
        println!("Read from shared memory: {}", read_msg);

        // Sleep again to simulate delay
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}