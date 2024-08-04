fn main() {
    let mut ptr = Box::new(10); // Allocate memory for an integer and initialize it to 10
    println!("{}", *ptr); // Print the value pointed to by ptr

    // Rust automatically deallocates the memory when ptr goes out of scope
}