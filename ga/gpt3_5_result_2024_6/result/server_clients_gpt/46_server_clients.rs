use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::{ptr, slice};
use std::{thread, time};
use libc::{c_void, c_int};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};

const SIZE: usize = 4096;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .unwrap();
    let fd = file.as_raw_fd();

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, SIZE as i64) } == -1 {
        panic!("ftruncate");
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(ptr::null_mut(), SIZE,
             ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
             MapFlags::MAP_SHARED, fd, 0)
            .expect("mmap")
    } as *mut c_void;

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).add(1024) };
        let message = CString::new("Test message").unwrap();
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe {
            CString::from_raw(p as *mut i8).into_string().unwrap()
        });
        thread::sleep(time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            CString::from_raw(p as *mut i8).into_string().unwrap()
        });
        thread::sleep(time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, SIZE) } == -1 {
        panic!("munmap");
    }

    drop(file); // Close the file

    // Remove the temporary file
    if unsafe { libc::unlink(CString::new("temp").unwrap().as_ptr() as *const i8) } == -1 {
        panic!("unlink");
    }
}