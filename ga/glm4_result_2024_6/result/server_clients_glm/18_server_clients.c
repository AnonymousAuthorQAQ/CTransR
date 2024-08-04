use std::fs;
use std::io;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = fs::File::create("temp").expect("Failed to create file");
    file.set_len(4096).expect("Failed to set file size");

    // Map the file into memory
    let map = unsafe { memmap::Mmap::map(&file).expect("Failed to map file") };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { map.as_ptr().add(1024) as *mut _ };
        unsafe { ptr::copy_nonoverlapping(b"Test message", p, 11) };
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const _).to_string_lossy()
        });
        sleep(Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr((map.as_ptr() as *const _, 1024).to_string_lossy()
        });
        stop += 1;
    }

    // Unmap and close everything
    drop(map);
    fs::remove_file("temp").expect("Failed to remove temp file");
}

#[cfg(unix)]
mod memmap {
    extern crate memmap;

    pub use memmap::Mmap;
}