fn main() {
    // Allocate memory for a single integer
    let ptr = Box::new(10);
    
    // Print the value before deallocation
    println!("{}", *ptr); // Outputs: 10

    // Box automatically deallocates the memory when it goes out of scope,
    // so we don't need to manually free it like in C.
}