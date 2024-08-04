fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10); // Box allocates memory on the heap

    // Print the value stored in the allocated memory
    println!("{}", *ptr); // Safe dereference

    // Memory is automatically freed when `ptr` goes out of scope
    // Uncommenting the next line would attempt to drop the box again, 
    // which is not allowed in Rust, as Box handles its own memory safely.
    // Box::from_raw(Box::into_raw(ptr)); // Would be an incorrect usage

    // Safe way to ensure the pointer cannot be used after being freed
    // `ptr` will be dropped automatically at the end of its scope.
}