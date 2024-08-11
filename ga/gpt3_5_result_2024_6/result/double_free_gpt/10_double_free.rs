fn main() {
    let mut ptr = Box::new(10);
    // Memory is automatically deallocated when 'ptr' goes out of scope
    // Attempting to manually free the memory is not necessary in Rust
    // Therefore, there is no equivalent to the 'free' function in Rust

    // Access the value stored in the memory location pointed to by 'ptr'
    println!("{}", *ptr);
}