use std::fs;
use std::io::{Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::slice;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let mut file = fs::File::create("temp").expect("Unable to create file");
    file.set_len(4096).expect("Unable to set file length");

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        eprintln!("mmap failed");
        return;
    }
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { slice::from_raw_parts_mut(ptr.offset(1024), 1024) };
        p.copy_from_slice(b"Test message\x00");
        println!("Message written to the shared memory: {}", str::from_utf8(p).unwrap());
        sleep(Duration::from_secs(1));

        // Read from the same offset
        let p = unsafe { slice::from_raw_parts(ptr.offset(1024), 1024) };
        println!("Read from shared memory: {}", str::from_utf8(p).unwrap());
        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        eprintln!("munmap failed");
    }

    file.sync_all().expect("Unable to sync file");
    fs::remove_file("temp").expect("Unable to remove file");
}