use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Write;
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
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.as_bytes().len());
        }
        println!("Message written to the shared memory: {}", std::str::from_utf8(unsafe {
            slice::from_raw_parts(p, message.as_bytes().len())
        }).unwrap());
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", std::str::from_utf8(unsafe {
            slice::from_raw_parts((ptr as *mut u8).add(1024), message.as_bytes().len())
        }).unwrap());
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(ptr, 4096);
    }

    drop(file);

    std::fs::remove_file("temp").expect("Failed to remove file");
}