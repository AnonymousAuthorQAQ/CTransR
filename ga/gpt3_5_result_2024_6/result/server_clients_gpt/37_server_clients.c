use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::ops::Deref;
use std::slice;
use std::{thread, time};
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::os::unix::io::IntoRawFd;
use libc::{c_void, c_char, off_t, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, MAP_ANONYMOUS, ftruncate};
use libc::{mmap, munmap};

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .expect("Failed to create file");
        
    let file_size: off_t = 4096;
    unsafe {
        let fd = file.as_raw_fd();
        if ftruncate(fd, file_size) == -1 {
            panic!("Failed to set file size");
        }

        let ptr = mmap(std::ptr::null_mut(), file_size as usize, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);

        if ptr == MAP_FAILED {
            panic!("Failed to map the file into memory");
        }
        
        let mut stop = 0;
        while stop < 10 {
             let p: *mut c_char = ptr.offset(1024);
             let c_str = CString::new("Test message").expect("Failed to create test message");
             let data = c_str.as_bytes_with_nul();
             let data_ptr: *const c_void = data.as_ptr() as *const c_void;
             std::ptr::copy(data_ptr, p as *mut c_void, data.len());
             
             let string = CString::from_raw(p);
             println!("Message written to the shared memory: {:?}", string.to_str().unwrap());
             thread::sleep(time::Duration::from_secs(1));
             
             let read_ptr: *const c_char = ptr.offset(1024) as *const c_char;
             let read_string = CString::from_raw(read_ptr as *mut c_char);
             println!("Read from shared memory: {:?}", read_string.to_str().unwrap());
             thread::sleep(time::Duration::from_secs(1));
             stop += 1;     
        }

        if munmap(ptr, file_size as usize) == -1 {
            panic!("Failed to unmap memory");
        }
    }
    
    file.into_raw_fd(); // release file descriptor
}