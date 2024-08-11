fn main() {
    let mut ptr = Box::new(10);
    
    // Memory is automatically deallocated when the Box goes out of scope,
    // so there is no need to manually free it in Rust.

    // Trying to access the value after it has been deallocated will result in a compile-time error.
    // Uncomment the line below to see the compiler error.
    // println!("{}", *ptr);
}