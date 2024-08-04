fn main() {
    // Allocate memory for an integer and initialize it to 10
    let ptr = Box::new(10);
    
    // Print the value using dereferencing
    println!("{}", *ptr);
    
    // Memory will be automatically freed when `ptr` goes out of scope
}