use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

use libc::{c_void, ftruncate, unlink};

fn main() {
    let path = Path::new("temp");
    let fd = match unsafe { libc::open(path.as_os_str().to_str().unwrap().as_ptr() as *const i8, libc::O_RDWR | libc::O_CREAT, 0o664) } {
        -1 => {
            eprintln!("Failed to open file: {}", io::Error::last_os_error());
            return;
        }
        fd => fd,
    };

    if unsafe { ftruncate(fd, 4096) } == -1 {
        eprintln!("Failed to set file size: {}", io::Error::last_os_error());
        unsafe {
            libc::close(fd);
        }
        return;
    }

    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        eprintln!("Failed to map memory: {}", io::Error::last_os_error());
        unsafe {
            libc::close(fd);
        }
        return;
    }

    let mut stop = 0;
    while stop < 10 {
        let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };
        let p = &mut slice[1024..];
        p.copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());
        sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", str::from_utf8(&slice[1024..]).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    if unsafe { libc::munmap(addr as *mut c_void, 4096) } == -1 {
        eprintln!("Failed to unmap memory: {}", io::Error::last_os_error());
    }

    unsafe {
        libc::close(fd);
        if unlink(path.as_os_str().to_str().unwrap()) == -1 {
            eprintln!("Failed to delete file: {}", io::Error::last_os_error());
        }
    }
}