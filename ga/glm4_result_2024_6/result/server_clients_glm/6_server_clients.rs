use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::str;
use std::time::Duration;

fn main() {
    let path = Path::new("temp");
    let mut file = fs::File::create(path).expect("Failed to create file");

    file.set_len(4096).expect("Failed to set file size");

    let mapping = unsafe {
        let addr = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        );
        if addr == libc::MAP_FAILED {
            panic!("Failed to map file");
        }
        addr
    };

    let stop = 10;
    let mut stop_count = 0;
    while stop_count < stop {
        let message = "Test message";
        unsafe {
            let slice = std::slice::from_raw_parts_mut(mapping as *mut u8, message.len());
            slice.copy_from_slice(message.as_bytes());
        }
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(
                mapping as *const u8,
                1024,
            ))
        });
        std::thread::sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(
                mapping as *const u8,
                1024,
            ))
        });
        std::thread::sleep(Duration::from_secs(1));
        stop_count += 1;
    }

    unsafe {
        if libc::munmap(mapping, 4096) == -1 {
            panic!("Failed to unmap file");
        }
    }

    file.sync_all().expect("Failed to sync file");
    fs::remove_file(path).expect("Failed to remove file");
}