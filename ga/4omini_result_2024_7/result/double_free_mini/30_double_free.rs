fn main() {
    // Dynamically allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);
    
    // Attempt to use the value
    println!("{}", *ptr);

    // The Box automatically frees memory when it goes out of scope,
    // so we can't manually free it like in C. However, we can simulate
    // a double-free by using `std::mem::drop` to drop the Box.
    drop(ptr); // This drops the Box, freeing the memory.

    // Uncommenting the following line will result in a compile-time error
    // because `ptr` has been dropped and cannot be used anymore.
    // println!("{}", *ptr); // This line would cause a compile error.

    // Rust does not allow double freeing and dereferencing after free,
    // which prevents undefined behavior common in C/C++.
}