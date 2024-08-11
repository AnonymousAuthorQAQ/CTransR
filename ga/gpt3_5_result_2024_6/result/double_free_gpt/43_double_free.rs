fn main() {
    let mut ptr = Box::new(10);
    // Rust uses smart pointers and automatic memory management, so we use Box instead of malloc
    // Box is a smart pointer that represents heap-allocated memory

    // Memory will be automatically deallocated when ptr goes out of scope
    // Therefore, no need to manually free the memory in Rust

    println!("{}", *ptr);
    // Output: 10
}