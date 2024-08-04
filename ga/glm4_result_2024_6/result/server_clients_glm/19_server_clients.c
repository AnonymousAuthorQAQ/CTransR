use std::fs::{File, OpenOptions};
use std::io::{Error, Result};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let path = Path::new("temp");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let fd = file.as_raw_fd();
    unsafe {
        if libc::ftruncate(fd, 4096) == -1 {
            return Err(Error::last_os_error());
        }
    }

    let addr = unsafe { libc::mmap(ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };

    if addr == libc::MAP_FAILED {
        return Err(Error::last_os_error());
    }

    let mut stop = 0;
    while stop < 10 {
        let p = addr as *mut u8;
        let message = b"Test message";
        unsafe {
            std::ptr::copy_nonoverlapping(message.as_ptr(), p.offset(1024), message.len());
        }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p.offset(1024) as *const i8)
                .to_string_lossy()
                .into_owned()
        });
        sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p.offset(1024) as *const i8)
                .to_string_lossy()
                .into_owned()
        });
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        if libc::munmap(addr as *mut libc::c_void, 4096) == -1 {
            return Err(Error::last_os_error());
        }
    }

    file.sync_all()?;

    std::fs::remove_file(path)?;

    Ok(())
}