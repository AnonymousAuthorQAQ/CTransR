fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);

    // dereferencing the pointer to get the value
    println!("{}", *ptr); // Print the value before simulating freeing

    // In Rust, memory is automatically managed, but let's simulate freeing 
    // by taking ownership of the Box
    let _freed_ptr = ptr; // This consumes the Box, simulating "freeing" it

    // Attempting to access freed memory (this will not compile)
    // println!("{}", *ptr); // Uncommenting this will cause a compile-time error since `ptr` is no longer valid

    // Instead, we could safely use an `Option` to demonstrate accessing it after
    let mut optional_val: Option<Box<i32>> = Some(Box::new(10));
    
    // Use its value before "removing" it
    if let Some(val) = optional_val {
        println!("{}", *val); // Accessing the value
    }

    // Simulate freeing the memory by dropping the Box
    optional_val = None; // This effectively drops the Box, similar to free()

    // At this point, optional_val is None and cannot be accessed safely
    // Attempting to access it like below would be unsafe and not allowed:
    // println!("{}", *optional_val.as_ref().unwrap()); // This would panic if uncommented
}