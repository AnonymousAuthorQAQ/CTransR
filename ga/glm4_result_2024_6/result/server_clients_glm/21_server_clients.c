use std::fs;
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let path = Path::new("temp");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    file.set_len(4096)?;

    let mmap = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if mmap == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let stop = 10;
    let mut stop_count = 0;

    while stop_count < stop {
        let msg = b"Test message";
        unsafe {
            std::ptr::copy(msg.as_ptr(), mmap.offset(1024), msg.len());
        }

        let view = unsafe { slice::from_raw_parts(mmap.offset(1024), msg.len()) };
        println!("Message written to the shared memory: {}", str::from_utf8(view).unwrap());

        sleep(Duration::from_secs(1));

        let view = unsafe { slice::from_raw_parts(mmap.offset(1024), msg.len()) };
        println!("Read from shared memory: {}", str::from_utf8(view).unwrap());

        sleep(Duration::from_secs(1));
        stop_count += 1;
    }

    unsafe {
        libc::munmap(mmap, 4096);
    }

    file.sync_all()?;

    fs::remove_file(path)?;

    Ok(())
}