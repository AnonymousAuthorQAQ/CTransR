fn main() {
    let mut ptr = Box::new(10);
    // Memory is automatically deallocated when ptr goes out of scope
    // No need to manually free memory in Rust

    // Attempting to access the value after deallocation will result in a compiler error
    // println!("{}", *ptr); // This will produce a compilation error

    println!("Memory deallocated successfully");
}