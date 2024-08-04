use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::str;
use std::thread::sleep;
use std::time::Duration;

use libc::{c_int, c_void, ftruncate, mmap, munmap};

const FILENAME: &str = "temp";
const FILESIZE: usize = 4096;
const OFFSET: usize = 1024;
const MESSAGE: &str = "Test message";

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILENAME)
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(FILESIZE as u64).expect("Failed to set file length");

    // Map the file into memory
    let addr = unsafe {
        mmap(
            ptr::null_mut(),
            FILESIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        panic!("Failed to map memory");
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        unsafe {
            let dst = addr.offset(OFFSET as isize);
            ptr::copy_nonoverlapping(MESSAGE.as_ptr(), dst as *mut u8, MESSAGE.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                addr.offset(OFFSET as isize) as *const u8,
                MESSAGE.len(),
            ))
        });

        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                addr.offset(OFFSET as isize) as *const u8,
                MESSAGE.len(),
            ))
        });

        sleep(Duration::from_secs(1));
    }

    // Unmap and close everything
    if unsafe { munmap(addr, FILESIZE) } == -1 {
        panic!("Failed to unmap memory");
    }
}

#[cfg(target_family = "unix")]
mod libc {
    extern "C" {
        pub fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: RawFd, offset: isize) -> *mut c_void;
        pub fn munmap(addr: *mut c_void, length: usize) -> c_int;
        pub fn ftruncate(fd: RawFd, length: isize) -> c_int;
    }
}