fn main() {
    let mut ptr = Box::new(10); // Allocate memory for an integer on the heap
    println!("{}", *ptr); // Print the value
    drop(ptr); // Free the memory
    // The following line would cause a compile-time error if uncommented
    // println!("{}", *ptr); // Attempt to print the value after it has been freed
}