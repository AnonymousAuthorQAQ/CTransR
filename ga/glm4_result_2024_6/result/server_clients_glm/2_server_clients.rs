use std::fs;
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file_path = Path::new("temp");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open(file_path)
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096).expect("Failed to set file length");

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
        eprintln!("Failed to map file");
        return;
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as usize + 1024) as *mut libc::c_char };
        let message = b"Test message\x00";
        unsafe { libc::strcpy(p, message.as_ptr() as *const libc::c_char) };
        println!("Message written to the shared memory: {}", unsafe {
            str::from_utf8_unchecked(message)
        });

        sleep(Duration::from_secs(1));

        // Read from the same offset
        let read_ptr = unsafe { (ptr as usize + 1024) as *const libc::c_char };
        println!("Read from shared memory: {}", unsafe {
            str::from_utf8_unchecked(libc::strcpy(read_ptr, ptr::null_mut()))
        });

        sleep(Duration::from_secs(1));
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr as *mut _, 4096) } == -1 {
        eprintln!("Failed to unmap file");
    }

    drop(file);
    fs::remove_file(file_path).expect("Failed to remove file");
}