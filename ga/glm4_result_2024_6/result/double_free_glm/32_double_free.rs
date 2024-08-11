fn main() {
    let mut ptr = Box::new(10); // Allocate memory for an integer and initialize it to 10
    let _ = ptr; // Drop the box to deallocate the memory
    let _ = ptr; // Attempt to drop the box again, which is invalid
    println!("{}", *ptr); // Attempt to print the value, which is undefined behavior
}
fn main() {
    let mut ptr = Box::new(10); // Allocate memory for an integer and initialize it to 10
    println!("{}", *ptr); // Print the value before deallocation
    let _ = ptr; // Drop the box to deallocate the memory
    // println!("{}", *ptr); // This line would cause an error since the memory has been deallocated
}