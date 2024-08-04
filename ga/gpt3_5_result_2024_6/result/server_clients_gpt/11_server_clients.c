use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::thread;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to open file");

    let fd = file.as_raw_fd();

    // Set the size of the file
    unsafe {
        if libc::ftruncate(fd, 4096) == -1 {
            panic!("ftruncate failed");
        }
    }

    // Map the file into memory
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = (ptr as isize + 1024) as *mut u8;
        
        unsafe {
            libc::strcpy(p as *mut i8, CString::new("Test message").unwrap().as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe { std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap() });

        thread::sleep(std::time::Duration::from_secs(1));

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe { std::ffi::CStr::from_ptr((ptr as isize + 1024) as *const i8).to_str().unwrap() });

        thread::sleep(std::time::Duration::from_secs(1));

        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if libc::munmap(ptr, 4096) == -1 {
            panic!("munmap failed");
        }
        libc::close(fd);
        libc::unlink(CString::new("temp").unwrap().as_ptr());
    }
}