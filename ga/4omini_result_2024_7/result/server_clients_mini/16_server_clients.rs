use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::thread;
use std::time::Duration;
use nix::fcntl::{open, OFlag};
use nix::sys::mman::{mmap, MunmapOptions, MapFlags, ProtFlags};
use nix::unistd::ftruncate;

fn main() -> io::Result<()> {
    // Create and open a file
    let fd: RawFd = open(
        "temp",
        OFlag::O_RDWR | OFlag::O_CREAT,
        nix::sys::stat::Mode::from_bits_truncate(0o664),
    ).expect("Failed to open file").as_raw_fd();

    // Set the size of the file
    ftruncate(fd, 4096).expect("Failed to truncate file");

    // Map the file into memory
    let ptr = mmap(
        ptr::null_mut(),
        4096,
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        MapFlags::MAP_SHARED,
        fd,
        0,
    ).expect("Failed to mmap");

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let offset_ptr = (ptr as *mut u8).add(1024);
        let message = "Test message";
        
        // Write the message to the shared memory
        unsafe {
            let bytes = message.as_bytes();
            ptr::copy_nonoverlapping(bytes.as_ptr(), offset_ptr, bytes.len());
            // Null-terminate the string
            *(offset_ptr.add(bytes.len())) = 0;
        }
        
        println!("Message written to the shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(offset_ptr as *const i8).to_string_lossy()
        });
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        println!("Read from shared memory: {}", unsafe {
            std::ffi::CStr::from_ptr(offset_ptr as *const i8).to_string_lossy()
        });
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    let _ = unsafe { MunmapOptions::new().direct_munmap(ptr, 4096) };
    
    // Remove the file
    std::fs::remove_file("temp")?;

    Ok(())
}