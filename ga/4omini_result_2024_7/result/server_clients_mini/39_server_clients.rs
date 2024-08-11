use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use libc::{c_void, mmap, munmap, PROT_READ, PROT_WRITE, MAP_SHARED};
use std::os::unix::fs::OpenOptionsExt;

const FILE_SIZE: usize = 4096;
const OFFSET: usize = 1024;

fn main() -> io::Result<()> {
    // Create and open a file
    let filename = "temp";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open(filename)?;

    // Set the size of the file
    file.set_len(FILE_SIZE as u64)?;

    // Map the file into memory
    let fd = file.as_raw_fd();
    let ptr = unsafe {
        mmap(
            ptr::null_mut(),
            FILE_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let message = CString::new("Test message").unwrap();
        unsafe {
            let p = (ptr as *mut u8).add(OFFSET);
            ptr::copy_nonoverlapping(message.as_ptr(), p, message.as_bytes().len());
            *p.add(message.as_bytes().len()) = 0; // Null-terminate the string
            println!("Message written to the shared memory: {:?}", slice::from_raw_parts(p, message.as_bytes().len()));
        }
        thread::sleep(Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        unsafe {
            let read_message = slice::from_raw_parts((ptr as *mut u8).add(OFFSET), 13); // 13 is the length of "Test message"
            println!("Read from shared memory: {:?}", String::from_utf8_lossy(read_message));
        }
        thread::sleep(Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // Unmap and close everything
    unsafe {
        if munmap(ptr, FILE_SIZE) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    std::fs::remove_file(filename)?;

    Ok(())
}