use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;
use libc::{c_void, mmap, munmap, shm_open, shm_unlink, O_RDWR, O_CREAT, PROT_READ, PROT_WRITE, MAP_SHARED};

const SHM_SIZE: usize = 4096;
const SHM_NAME: &str = "/temp";

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = unsafe {
        shm_open(
            CString::new(SHM_NAME).unwrap().as_ptr(),
            O_RDWR | O_CREAT,
            0o664,
        )
    };
    
    if fd < 0 {
        eprintln!("Failed to open shared memory segment");
        return Err(io::Error::last_os_error());
    }

    // Set the size of the file
    if unsafe { libc::ftruncate(fd, SHM_SIZE as i64) } == -1 {
        eprintln!("Failed to truncate file");
        return Err(io::Error::last_os_error());
    }

    // Map the file into memory
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            SHM_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        eprintln!("Failed to mmap");
        return Err(io::Error::last_os_error());
    }

    // Main loop
    for _ in 0..10 {
        // Write to a specific offset within the memory-mapped area
        let message = "Test message";
        let p = unsafe { (ptr as *mut u8).add(1024) };
        unsafe {
            let message_cstr = CString::new(message).unwrap();
            let bytes = message_cstr.as_bytes_with_nul();
            ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
        }

        println!("Message written to the shared memory: {}", unsafe {
            CString::from_raw(p as *mut i8).to_string_lossy()
        });

        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay

        // Read from the same offset
        println!(
            "Read from shared memory: {}",
            unsafe { CString::from_raw(p as *mut i8).to_string_lossy() }
        );
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
    }

    // Unmap and close everything
    if unsafe { munmap(ptr, SHM_SIZE) } == -1 {
        eprintln!("Failed to munmap");
        return Err(io::Error::last_os_error());
    }

    // Cleanup: unlink the shared memory segment
    if unsafe { shm_unlink(CString::new(SHM_NAME).unwrap().as_ptr()) } == -1 {
        eprintln!("Failed to unlink shared memory");
        return Err(io::Error::last_os_error());
    }

    Ok(())
}