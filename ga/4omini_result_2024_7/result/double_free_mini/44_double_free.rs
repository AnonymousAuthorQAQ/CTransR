fn main() {
    // Allocate memory for an integer on the heap
    let ptr = Box::new(10);

    // Print the value stored in the box
    println!("{}", ptr);

    // The Box will automatically be freed when it goes out of scope
    // There's no need to manually free memory like in C.
    
    // Uncommenting the next line would cause a compile-time error in Rust,
    // because `ptr` has already been moved to `Box` and cannot be used again.
    // println!("{}", *ptr); // This would not compile.

    // If you want to mimic double freeing, you can create a scenario
    // but it will not compile since Rust's ownership model prevents it.
}