fn main() {
    let mut ptr = Box::new(10);
    drop(ptr);
    drop(ptr); // This line is not needed in Rust as Box automatically deallocates memory when it goes out of scope
    // println!("{}", *ptr); // Attempting to access the value after deallocation would result in a compile-time error in Rust
}