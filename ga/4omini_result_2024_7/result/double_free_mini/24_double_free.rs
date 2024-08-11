fn main() {
    // Allocate memory for an integer
    let mut ptr: Box<i32> = Box::new(10);
    
    // Print the value before freeing
    println!("{}", *ptr); // This will safely print 10

    // Freeing the memory is automatic when the Box goes out of scope.
    // Rust will automatically free the memory when `ptr` goes out of scope.
    // To mimic the double free scenario, we can drop it manually.
    
    // Uncommenting the following line would mimic a double free scenario,
    // but it would cause a compile-time error because Rust prevents double free.
    // std::mem::drop(ptr);

    // Now, ptr cannot be used after it's dropped, which prevents undefined behavior.
    // Uncommenting the following line would result in a compile-time error:
    // println!("{}", *ptr); // This line would cause a compile-time error in Rust.

    // However, if we want to show a similar error concept, we can create a new scope:
    {
        let _ptr = Box::new(10); // Allocating memory
        // _ptr goes out of scope and is dropped here
    } // Memory is freed here, and we cannot access _ptr anymore

    // Uncommenting the following line would also result in a compile-time error:
    // println!("{}", *_ptr); // This line would cause a compile-time error in Rust.
}