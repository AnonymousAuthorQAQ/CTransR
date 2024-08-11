use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
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
        unsafe {
            let data_ptr = ptr.offset(1024);
            let data_slice = slice::from_raw_parts_mut(data_ptr as *mut u8, 12);
            let message = b"Test message";
            data_slice.copy_from_slice(message);

            let data = CString::from_raw(data_ptr as *mut i8).into_string().unwrap();
            println!("Message written to the shared memory: {}", data);
        }

        sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        unsafe {
            let data = CString::from_raw(ptr as *mut i8).into_string().unwrap();
            println!("Read from shared memory: {}", &data[1024..]);
        }

        sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        libc::munmap(ptr, 4096);
    }

    drop(file);

    std::fs::remove_file("temp").expect("Failed to remove file");
}