use std::ffi::CString;
use std::io;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::mem;
use std::thread;
use std::time::Duration;

use nix::libc::{c_void, c_char, off_t};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::fcntl::{open, OFlag};
use nix::unistd::{close, unlink};

fn main() -> io::Result<()> {
    // Create and open a file
    let fd = open("temp", OFlag::O_RDWR | OFlag::O_CREAT, 0o664)?;

    // Set the size of the file
    ftruncate(fd, 4096)?;

    // Map the file into memory
    let ptr = mmap(0 as *mut c_void, 4096, ProtFlags::PROT_READ | ProtFlags::PROT_WRITE, 
                   MapFlags::MAP_SHARED, fd, 0)?;

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let p = unsafe { ((ptr as *mut c_char).offset(1024)) };
        let message = CString::new("Test message")?;
        unsafe {
            ptr::copy_nonoverlapping(message.as_ptr(), p as *mut u8, message.as_bytes().len());
        }
        println!("Message written to the shared memory: {}", message.to_str()?);
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_message = unsafe { slice::from_raw_parts(ptr as *mut c_char, 4096) };
        let read_message_str = CString::from_vec_unchecked(read_message.to_vec()).to_str()?;
        println!("Read from shared memory: {}", read_message_str);
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    munmap(ptr, 4096)?;
    close(fd)?;

    unlink("temp")?;

    Ok(())
}