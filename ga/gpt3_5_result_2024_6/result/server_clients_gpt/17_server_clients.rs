use std::ffi::CString;
use std::os::raw::c_char;
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::os::raw::c_void;
use std::ptr;
use std::slice;
use std::{thread, time};
use std::io::{Write, Result};
use libc::{c_int, c_void};
use std::mem::size_of;

fn main() {
    let file = OpenOptions::new().read(true).write(true).create(true).open("temp").unwrap();
    let file_size = 4096;
    file.set_len(file_size as u64).unwrap();

    let ptr = unsafe {
        let ptr = libc::mmap(
            ptr::null_mut(),
            file_size as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("mmap failed");
        }
        ptr
    };

    let mut stop = 0;
    while stop < 10 {
        let message = CString::new("Test message").unwrap();
        unsafe {
            let p = ptr.offset(1024);
            libc::strcpy(p as *mut c_char, message.as_ptr());
            println!("Message written to the shared memory: {}", CString::from_raw(p as *mut c_char).to_str().unwrap());
            thread::sleep(time::Duration::from_secs(1));

            println!("Read from shared memory: {}", CString::from_raw(p as *mut c_char).to_str().unwrap());
            thread::sleep(time::Duration::from_secs(1));
        }
        stop += 1;
    }

    unsafe {
        if libc::munmap(ptr, file_size) == -1 {
            panic!("munmap failed");
        }
    }

    drop(file);

    std::fs::remove_file("temp").unwrap();
}