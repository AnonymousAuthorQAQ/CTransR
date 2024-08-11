use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() {
    // Create and open a file
    let fd = unsafe {
        libc::open(CString::new("temp").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o644)
    };
    if fd == -1 {
        panic!("open failed");
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, 4096) } == -1 {
        panic!("ftruncate failed");
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
        let p = unsafe { (ptr as *mut libc::c_char).offset(1024) };
        unsafe {
            libc::strcpy(p, b"Test message\0".as_ptr() as *const libc::c_char);
        }
        println!("Message written to the shared memory: {}", unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts(p as *const u8, 12))
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts((ptr as *const libc::c_char).offset(1024) as *const u8, 12))
        });

        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        panic!("munmap failed");
    }

    unsafe {
        libc::close(fd);
    }

    if unsafe { libc::unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        panic!("unlink failed");
    }
}