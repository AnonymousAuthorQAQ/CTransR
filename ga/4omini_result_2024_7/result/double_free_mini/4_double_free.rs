fn main() {
    // Allocate memory on the heap for an i32 and initialize it to 10
    let ptr: Box<i32> = Box::new(10);
    
    // Print the value before dropping
    println!("{}", *ptr);
    
    // Droping the box, deallocating the memory
    // Note: In Rust, this happens implicitly at the end of its scope,
    // but we can explicitly drop it here for clarity.
    drop(ptr);

    // The following access would normally cause a compile error in Rust due to ownership rules.
    // However, we will use unsafe code to mimic freed memory access, which is not recommended.
    // Unsafe code should be used with caution.
    unsafe {
        // Create a raw pointer to mimic 'freeing' behavior (not recommended for regular use)
        let raw_ptr: *mut i32 = std::ptr::null_mut(); // Set to null to avoid dangling pointer
        let invalid_value = if let Some(value) = raw_ptr.as_ref() {
            *value
        } else {
            -1 // Return a default value instead, to mimic undefined behavior
        };

        println!("{}", invalid_value); // Handling potential invalid access
    }
}