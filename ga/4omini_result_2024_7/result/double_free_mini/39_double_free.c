fn main() {
    // Allocate memory for an `i32` and initialize it with the value 10
    let ptr = Box::new(10);

    // Print the value pointed to by ptr
    println!("{}", ptr); // This safely prints the value

    // The Box will automatically free the memory when it goes out of scope
}