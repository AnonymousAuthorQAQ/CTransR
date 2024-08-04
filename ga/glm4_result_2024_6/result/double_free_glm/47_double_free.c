fn main() {
    let mut ptr = Box::new(10); // Allocate memory for an integer on the heap
    println!("{}", *ptr); // Print the value before freeing the memory
    drop(ptr); // Free the memory
    // ptr is now dropped and cannot be used again
}