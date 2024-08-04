use std::ptr;

fn main() {
    // Allocate memory for an integer
    let ptr: *mut i32 = unsafe { libc::malloc(std::mem::size_of::<i32>()) as *mut i32 };
    
    // Check if memory allocation was successful
    if ptr.is_null() {
        panic!("Memory allocation failed");
    }

    // Assign the value 10 to the allocated memory
    unsafe {
        *ptr = 10;
    }

    // Free the allocated memory
    unsafe {
        libc::free(ptr as *mut libc::c_void);
    }

    // Attempt to free the memory again (unsafe and leads to undefined behavior)
    unsafe {
        libc::free(ptr as *mut libc::c_void); // This is incorrect in practice
    }

    // Attempt to read the value from the freed memory (undefined behavior)
    unsafe {
        println!("{}", *ptr); // This is also incorrect in practice
    }
}