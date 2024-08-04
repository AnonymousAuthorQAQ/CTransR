fn main() {
    // Allocate memory for an integer
    let mut value = Box::new(10);
    
    // Print the value
    println!("{}", *value);
    
    // Value is automatically dropped when it goes out of scope
    // There is no need to explicitly free memory in Rust
}