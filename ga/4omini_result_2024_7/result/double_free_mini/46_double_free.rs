fn main() {
    // Allocate memory for an integer using Box
    let ptr: Box<i32> = Box::new(10);
    
    // Print the value that ptr points to
    println!("{}", *ptr);
    
    // Memory is automatically freed when ptr goes out of scope
    // No need to explicitly free memory in Rust
}