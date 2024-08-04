fn main() {
    let mut ptr = Box::new(10); // Allocate memory on the heap for an integer and initialize it to 10
    println!("{}", *ptr); // Print the value
    drop(ptr); // Drop the Box to deallocate the memory
    // ptr is now dropped and cannot be used
}