use std::io::{Error, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let path = Path::new("temp");
    let fd = unsafe { libc::open(path.as_os_str().as_bytes().as_ptr() as *const libc::c_char, libc::O_RDWR | libc::O_CREAT, 0664) };
    if fd < 0 {
        Err(Error::last_os_error())
    } else {
        let mut f = unsafe { std::fs::File::from_raw_fd(fd) };
        f.set_len(4096)?;
        let ptr = unsafe { libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };
        if ptr == libc::MAP_FAILED {
            Err(Error::last_os_error())
        } else {
            let mut stop = 0;
            while stop < 10 {
                let p = unsafe { slice::from_raw_parts_mut(ptr.offset(1024), 1024) };
                p.copy_from_slice(b"Test message\x00");
                println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());
                sleep(Duration::from_secs(1));

                let p = unsafe { slice::from_raw_parts(ptr.offset(1024), 1024) };
                println!("Read from shared memory: {}", str::from_utf8(p).unwrap());
                sleep(Duration::from_secs(1));
                stop += 1;
            }

            unsafe { libc::munmap(ptr, 4096) };
            f.set_len(0)?;
            std::fs::remove_file(path)?;
            Ok(())
        }
    }
}