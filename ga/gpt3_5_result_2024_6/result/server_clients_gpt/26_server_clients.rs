use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

fn main() {
    // Create and open a file
    let fd = unsafe {
        libc::open(CString::new("temp").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o664)
    };
    if fd == -1 {
        eprintln!("open");
        std::process::exit(1);
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, 4096) } == -1 {
        eprintln!("ftruncate");
        std::process::exit(1);
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
        eprintln!("mmap");
        std::process::exit(1);
    }

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { (ptr as *mut u8).offset(1024) };
        unsafe {
            libc::strcpy(p as *mut i8, CString::new("Test message").unwrap().as_ptr());
        }
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(p as *const i8).to_str().unwrap()
        });
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!(
            "Read from shared memory: {}",
            unsafe { std::ffi::CStr::from_ptr(ptr as *const i8).to_str().unwrap() }
        );
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    if unsafe { libc::munmap(ptr, 4096) } == -1 {
        eprintln!("munmap");
        std::process::exit(1);
    }

    unsafe {
        libc::close(fd);
    }

    if unsafe { libc::unlink(CString::new("temp").unwrap().as_ptr()) } == -1 {
        eprintln!("unlink");
        std::process::exit(1);
    }
}