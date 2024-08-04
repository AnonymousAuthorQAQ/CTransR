use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ffi::CStr;

fn main() {
    let path = Path::new("temp");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    file.set_len(4096).expect("Failed to set file length");

    let mut mmap = unsafe {
        let addr = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );
        if addr == libc::MAP_FAILED {
            panic!("Failed to mmap");
        }
        addr
    };

    let stop = 10;
    for _ in 0..stop {
        let msg = CString::new("Test message").unwrap();
        unsafe {
            ptr::copy(msg.as_ptr(), mmap.offset(1024), msg.as_bytes_with_nul().len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(mmap.offset(1024), 12))
        });
        std::thread::sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(mmap.offset(1024), 12))
        });
        std::thread::sleep(Duration::from_secs(1));
    }

    unsafe {
        libc::munmap(mmap, 4096);
    }

    file.sync_all().expect("Failed to sync file");
    std::fs::remove_file(path).expect("Failed to remove file");
}