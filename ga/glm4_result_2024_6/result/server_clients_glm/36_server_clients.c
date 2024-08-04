use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
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

    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let mut stop = 0;
    while stop < 10 {
        let p = unsafe { slice::from_raw_parts_mut(addr as *mut _, 4096) };
        p[1024..].copy_from_slice(b"Test message");
        println!("Message written to the shared memory: {}", str::from_utf8(&p[1024..]).unwrap());
        sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", str::from_utf8(&p[1024..]).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    unsafe {
        if libc::munmap(addr, 4096) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    file.close()?;
    std::fs::remove_file(path)?;

    Ok(())
}