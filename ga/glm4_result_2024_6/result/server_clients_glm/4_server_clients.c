use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::str;
use std::time::Duration;

fn main() {
    let path = Path::new("temp");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    file.set_len(4096).unwrap();

    let addr = unsafe { mmap(
        ptr::null_mut(),
        4096,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        file.as_raw_fd(),
        0,
    ) };

    if addr == libc::MAP_FAILED {
        eprintln!("mmap failed");
        return;
    }

    let mut stop = 0;
    while stop < 10 {
        let msg = "Test message";
        unsafe {
            ptr::copy(msg.as_ptr(), addr.offset(1024), msg.len());
        }

        let mut buffer = [0; 12];
        unsafe {
            ptr::copy(addr.offset(1024), buffer.as_mut_ptr(), msg.len());
        }
        println!("Message written to the shared memory: {}", str::from_utf8(&buffer).unwrap());

        std::thread::sleep(Duration::from_secs(1));

        let mut buffer = [0; 12];
        unsafe {
            ptr::copy(addr.offset(1024), buffer.as_mut_ptr(), msg.len());
        }
        println!("Read from shared memory: {}", str::from_utf8(&buffer).unwrap());

        std::thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        munmap(addr, 4096).unwrap();
    }

    file.sync_all().unwrap();
    std::fs::remove_file(path).unwrap();
}

extern "C" {
    fn mmap(addr: *mut libc::c_void, len: usize, prot: libc::c_int, flags: libc::c_int, fd: RawFd, offset: libc::off_t) -> *mut libc::c_void;
    fn munmap(addr: *mut libc::c_void, len: usize) -> libc::c_int;
}