use std::ffi::CString;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use libc::{c_void, c_char, off_t, size_t, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, O_RDWR, O_CREAT, ftruncate, open, close, unlink, perror, sleep, strlen, strcpy, memcpy, mlock, munmap, mmap};

fn main() -> io::Result<()> {
    unsafe {
        let c_file = CString::new("temp")?;
        // Open and create the file
        let fd = open(c_file.as_ptr(), O_RDWR | O_CREAT, 0o644);
        if fd == -1 {
            perror("open\0".as_ptr() as *const _);
            return Err(io::Error::last_os_error());
        }

        // Set the size of the file
        if ftruncate(fd, 4096) == -1 {
            perror("ftruncate\0".as_ptr() as *const _);
            return Err(io::Error::last_os_error());
        }

        // Map the file into memory
        let ptr = mmap(ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
        if ptr == MAP_FAILED {
            perror("mmap\0".as_ptr() as *const _);
            return Err(io::Error::last_os_error());
        }

        // Main loop
        let mut stop = 0;
        while stop < 10 {
            // Write to a specific offset within the memory-mapped area
            let p: *mut c_char = ptr.offset(1024);
            let message = "Test message\0";
			memcpy(p as *mut c_void, message.as_ptr() as *const c_void, strlen(message) + 1);
			println!("Message written to the shared memory: {:?}", CString::from_raw(p).to_str().unwrap());
            sleep(1);

            // Read from the same offset
            println!("Read from shared memory: {:?}", CString::from_raw(ptr as *mut c_char).to_str().unwrap());
            sleep(1);

            stop += 1;
        }

        // Unmap and close everything
        if munmap(ptr, 4096) == -1 {
            perror("munmap\0".as_ptr() as *const _);
            return Err(io::Error::last_os_error());
        }

        close(fd);
        
        let c_filename = CString::new("temp").unwrap();
        if unlink(c_filename.as_ptr()) == -1 {
            perror("unlink\0".as_ptr() as *const _);
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}