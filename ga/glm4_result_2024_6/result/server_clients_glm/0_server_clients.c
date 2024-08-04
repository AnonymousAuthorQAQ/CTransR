use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("Failed to map memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { ptr.offset(1024) as *mut libc::c_char };
        unsafe {
            libc::strcpy(p, b"Test message\0".as_ptr() as *const libc::c_char);
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(p as *const u8, 10))
        });
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(p as *const u8, 10))
        });
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        panic!("Failed to unmap memory");
    }

    file.close().expect("Failed to close file");

    if std::path::Path::new("temp").exists() {
        std::fs::remove_file("temp").expect("Failed to remove file");
    }
}

#[link(name = "c")]
extern "C" {
    pub fn mmap(addr: *mut libc::c_void, len: usize, prot: libc::c_int, flags: libc::c_int, fd: RawFd, offset: isize) -> *mut libc::c_void;
    pub fn munmap(addr: *mut libc::c_void, len: usize) -> libc::c_int;
    pub fn strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
}