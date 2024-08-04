use std::io::{self, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let fd = unsafe { libc::open("temp".as_ptr() as *const libc::c_char, libc::O_RDWR | libc::O_CREAT, 0664) };
    if fd == -1 {
        eprintln!("Failed to open file: {}", io::Error::last_os_error());
        return;
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, 4096) } == -1 {
        eprintln!("Failed to truncate file: {}", io::Error::last_os_error());
        unsafe {
            libc::close(fd);
        }
        return;
    }

    // Map the file into memory
    let addr = unsafe { libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };
    if addr == libc::MAP_FAILED {
        eprintln!("Failed to map file: {}", io::Error::last_os_error());
        unsafe {
            libc::close(fd);
        }
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = addr.offset(1024);
        let msg = b"Test message\x00";
        unsafe {
            ptr::copy_nonoverlapping(msg.as_ptr(), p, msg.len());
        }
        println!("Message written to the shared memory: {}", str::from_utf8(&msg).unwrap());
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let slice = unsafe { slice::from_raw_parts(p, msg.len()) };
        println!("Read from shared memory: {}", str::from_utf8(slice).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(addr, 4096) } == -1 {
        eprintln!("Failed to unmap file: {}", io::Error::last_os_error());
    }

    unsafe {
        libc::close(fd);
    }

    if std::fs::remove_file("temp").is_err() {
        eprintln!("Failed to remove file: {}", io::Error::last_os_error());
    }
}

#[link(name = "c")]
extern "C" {
    fn open(name: *const libc::c_char, flags: libc::c_int, mode: libc::mode_t) -> libc::c_int;
    fn ftruncate(fd: libc::c_int, length: libc::off_t) -> libc::c_int;
    fn close(fd: libc::c_int) -> libc::c_int;
    fn munmap(addr: *mut libc::c_void, length: libc::size_t) -> libc::c_int;
    fn mmap(addr: *mut libc::c_void, length: libc::size_t, prot: libc::c_int, flags: libc::c_int, fd: libc::c_int, offset: libc::off_t) -> *mut libc::c_void;
}