fn main() {
    let mut ptr = Box::new(10);
    println!("Value before free: {}", *ptr);
    
    // Intentionally double-free to demonstrate the concept
    // This is undefined behavior and should be avoided in real code
    drop(ptr);
    drop(ptr);
    
    // Attempt to print the value, which will cause a panic due to double-free
    println!("Value after free: {}", *ptr);
}