fn main() {
    // Allocate memory on the heap and store a value
    let mut ptr = Box::new(10);

    // Print the value
    println!("{}", *ptr);

    // Move the ownership to the end of the scope
    // Rust will automatically free memory when 'ptr' goes out of scope
    drop(ptr); // Optional: Explicitly drop the box, memory is freed here

    // After dropping, 'ptr' cannot be used anymore
    // Uncommenting the next line will cause a compile-time error
    // println!("{}", *ptr);
}