use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Failed to open file");

    // Set the size of the file
    file.set_len(4096)
        .expect("Failed to set file size");

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
        panic!("Failed to map memory");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).add(1024) };
        let message = CString::new("Test message").expect("Failed to create CString");
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe {
            String::from_utf8_lossy(slice::from_raw_parts(p, 12))
        });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            String::from_utf8_lossy(slice::from_raw_parts((ptr as *mut u8).add(1024), 12))
        });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap memory
    unsafe {
        libc::munmap(ptr, 4096);
    }

    // Close file
    drop(file);

    // Remove file
    if let Err(e) = std::fs::remove_file("temp") {
        eprintln!("Error removing file: {}", e);
    }
}