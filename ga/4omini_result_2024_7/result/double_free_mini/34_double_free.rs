fn main() {
    let ptr = Box::new(10); // Allocate memory for an integer and store 10
    println!("{}", ptr); // Print the value stored in the box
    // No need to manually free the memory; Box will handle it automatically when it goes out of scope
}