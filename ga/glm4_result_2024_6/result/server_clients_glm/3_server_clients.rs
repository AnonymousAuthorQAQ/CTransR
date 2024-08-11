use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let path = Path::new("temp");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    file.set_len(4096)?;

    let fd = file.as_raw_fd();
    let ptr = unsafe { libc::mmap(std::ptr::null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0) };

    if ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let mut stop = 0;
    while stop < 10 {
        let p = unsafe { (ptr as *mut u8).offset(1024) as *mut libc::c_char };
        unsafe { libc::strcpy(p, b"Test message\0" as *const u8 as *const libc::c_char); }

        let msg = unsafe { str::from_utf8_unchecked(std::slice::from_raw_parts(p as *const u8, 12)) };
        println!("Message written to the shared memory: {}", msg);

        sleep(Duration::from_secs(1));

        let msg = unsafe { str::from_utf8_unchecked(std::slice::from_raw_parts(p as *const u8, 12)) };
        println!("Read from shared memory: {}", msg);

        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe { libc::munmap(ptr as *mut libc::c_void, 4096); }

    file.close()?;

    std::fs::remove_file(path)?;

    Ok(())
}