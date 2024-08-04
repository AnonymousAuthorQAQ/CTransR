fn main() {
    let mut ptr = Box::new(10);
    // Box::new allocates memory on the heap and stores the value 10 in it
    
    // Memory is automatically deallocated when 'ptr' goes out of scope
    // No explicit call to 'free' is needed in Rust
    
    // Attempting to access the value stored at the memory location
    println!("{}", *ptr);
}