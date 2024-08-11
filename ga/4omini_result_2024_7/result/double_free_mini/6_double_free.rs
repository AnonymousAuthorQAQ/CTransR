fn main() {
    // Allocate memory for an integer
    let mut ptr: Box<i32> = Box::new(10);
    
    // Freeing the memory (Box will be dropped automatically at the end of scope)
    // In Rust, we don't explicitly call free; Box takes care of it.
    let value = *ptr; // Get the value to print before it goes out of scope.
    std::mem::drop(ptr); // Explicitly drop the Box (free the memory).

    // Attempting to access ptr after it has been dropped would lead to a compile-time error.
    // So, we cannot perform `println!("{}", *ptr)` directly.
    
    // Print the value we had before dropping the Box
    println!("{}", value);
}