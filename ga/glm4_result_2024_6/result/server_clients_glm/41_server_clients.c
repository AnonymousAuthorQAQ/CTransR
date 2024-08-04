use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

use libc::{c_void, ftruncate, mmap, munmap, open, close, unlink, O_RDWR, O_CREAT, S_IWUSR, S_IRUSR, PROT_READ, PROT_WRITE, MAP_SHARED};

fn main() -> io::Result<()> {
    let path = Path::new("temp");
    let fd = unsafe { open(path.as_os_str().as_bytes(), O_RDWR | O_CREAT, S_IWUSR | S_IRUSR) };
    if fd == -1 {
        eprintln!("Error opening file: {}", io::Error::last_os_error());
        return Err(io::Error::last_os_error());
    }

    if unsafe { ftruncate(fd, 4096) } == -1 {
        eprintln!("Error setting file size: {}", io::Error::last_os_error());
        unsafe { close(fd); }
        return Err(io::Error::last_os_error());
    }

    let addr = unsafe { mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if addr == libc::MAP_FAILED {
        eprintln!("Error mapping file: {}", io::Error::last_os_error());
        unsafe { close(fd); }
        return Err(io::Error::last_os_error());
    }

    let mut stop = 0;
    while stop < 10 {
        let p = unsafe { slice::from_raw_parts_mut(addr.offset(1024), 1024) };
        str::from_utf8(b"Test message").unwrap().as_bytes().copy_to_slice(p);
        println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());
        std::thread::sleep(Duration::from_secs(1));

        let p = unsafe { slice::from_raw_parts(addr.offset(1024), 1024) };
        println!("Read from shared memory: {}", str::from_utf8(p).unwrap());
        std::thread::sleep(Duration::from_secs(1));
        stop += 1;
    }

    if unsafe { munmap(addr, 4096) } == -1 {
        eprintln!("Error unmapping memory: {}", io::Error::last_os_error());
    }

    unsafe { close(fd); }

    if unsafe { unlink(path.as_os_str().as_bytes()) } == -1 {
        eprintln!("Error deleting file: {}", io::Error::last_os_error());
    }

    Ok(())
}