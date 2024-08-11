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
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;

fn main() {
    // Create and open a file
    let path = Path::new("temp");
    let fd = open(path, OFlag::RDWR | OFlag::CREAT, Mode::empty()).unwrap();
    let file = fs::File::open(path).unwrap();

    // Set the size of the file
    file.set_len(4096).unwrap();

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { slice::from_raw_parts_mut(ptr.offset(1024), 1024) };
        p.copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let p = unsafe { slice::from_raw_parts(ptr.offset(1024), 1024) };
        println!("Read from shared memory: {}", str::from_utf8(p).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        munmap(ptr, 4096).unwrap();
    }

    file.close().unwrap();

    fs::remove_file(path).unwrap();
}