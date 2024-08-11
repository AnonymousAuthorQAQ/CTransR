use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use libc::{mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE, MAP_FAILED, c_void};
use std::io::Write;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .expect("Could not open file");

    // Set the size of the file
    file.set_len(4096)
        .expect("Could not set file size");

    // Map the file into memory
    let fd = file.as_raw_fd();
    let ptr = unsafe {
        mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    };

    if ptr == MAP_FAILED {
        panic!("Error mapping file into memory");
    }

    // Main loop
    let mut stop = 0;
    loop {
        let offset = 1024;

        // Write to a specific offset within the memory-mapped area
        let p = ptr.offset(offset as isize) as *mut u8;
        let message = CString::new("Test message").expect("CString conversion failed");
        unsafe {
            libc::strcpy(p as *mut i8, message.as_ptr());
        }
        
        let msg = unsafe {
            std::slice::from_raw_parts(ptr.offset(offset as isize) as *const u8, "Test message".len())
        };

        println!("Message written to shared memory: {:?}", std::str::from_utf8(msg).unwrap());
        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_msg = unsafe {
            std::slice::from_raw_parts(ptr.offset(offset as isize) as *const u8, "Test message".len())
        };

        println!("Read from shared memory: {:?}", std::str::from_utf8(read_msg).unwrap());
        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;

        if stop == 10 {
            break;
        }
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, 4096) == -1 {
            panic!("Error unmapping memory");
        }
    }

    drop(file); // Close the file and delete

    std::fs::remove_file("temp")
        .expect("Unable to delete file");
}