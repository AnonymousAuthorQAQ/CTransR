fn main() {
    // Allocate memory for an integer on the heap
    let ptr = Box::new(10); // `Box` is a smart pointer that manages heap memory
    
    // Print the value
    println!("{}", *ptr); // Dereference to get the value
    
    // `Box` will automatically free the memory when it goes out of scope
}