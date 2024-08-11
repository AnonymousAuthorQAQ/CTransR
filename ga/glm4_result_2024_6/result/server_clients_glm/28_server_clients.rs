use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

const FILE_SIZE: usize = 4096;
const OFFSET: usize = 1024;
const MESSAGE: &[u8] = b"Test message";

fn main() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    file.set_len(FILE_SIZE as u64).expect("Failed to set file size");

    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            FILE_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        panic!("Failed to map file");
    }

    let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, FILE_SIZE) };

    for _ in 0..10 {
        slice[OFFSET..OFFSET + MESSAGE.len()].copy_from_slice(MESSAGE);
        println!("Message written to the shared memory: {}", str::from_utf8(&slice[OFFSET..OFFSET + MESSAGE.len()]).unwrap());
        sleep(Duration::from_secs(1));

        println!("Read from shared memory: {}", str::from_utf8(&slice[OFFSET..OFFSET + MESSAGE.len()]).unwrap());
        sleep(Duration::from_secs(1));
    }

    if unsafe { libc::munmap(addr as *mut libc::c_void, FILE_SIZE) } == -1 {
        panic!("Failed to unmap file");
    }

    drop(file);

    if std::fs::remove_file("temp").is_err() {
        panic!("Failed to remove file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}