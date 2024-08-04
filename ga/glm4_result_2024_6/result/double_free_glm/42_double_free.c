fn main() {
    let mut ptr = Box::new(10);
    // Rust automatically frees the memory when the Box goes out of scope
    // Attempting to free it again or access it after going out of scope would be a compile-time error
    
    // Rust does not support printing the value of the Box after it has been dropped
    // println!("{}", *ptr); // This line would cause a compile-time error

    // Rust also does not allow dangling references, so the Box cannot be used after it is dropped
    // Accessing the value of the Box after it has been dropped would be a compile-time error
}