use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

use libc::{c_void, ftruncate, mmap, munmap, unlink};
use memmap::MmapMut;

fn main() {
    let path = Path::new("temp");
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    file.set_len(4096).expect("Failed to set file length");

    let mut mmap = unsafe {
        MmapMut::map_mut(&file).expect("Failed to map file")
    };

    let mut stop = 0;
    while stop < 10 {
        let msg = b"Test message";
        let dst = &mut mmap[1024..1024 + msg.len()];
        dst.copy_from_slice(msg);

        println!("Message written to the shared memory: {}", str::from_utf8(dst).unwrap());
        sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", str::from_utf8(&mmap[1024..1024 + msg.len()]).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // No need to unmap or close the file, it will be automatically handled when `mmap` goes out of scope
    // Also, the file will not be deleted as Rust does not have an equivalent to `unlink`
}