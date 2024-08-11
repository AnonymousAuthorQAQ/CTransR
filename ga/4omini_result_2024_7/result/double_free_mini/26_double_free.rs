fn main() {
    let ptr: *mut i32 = unsafe { libc::malloc(std::mem::size_of::<i32>()) as *mut i32 };
    
    if !ptr.is_null() {
        unsafe {
            *ptr = 10; // Assigning value to the allocated memory
            println!("{}", *ptr); // Print the value
            
            // Remember to free the allocated memory
            libc::free(ptr as *mut libc::c_void);
        }
    }
}