fn main() {
    // Allocate memory for an integer using Box (heap allocation)
    let ptr = Box::new(10);

    // Print the value
    println!("{}", ptr);

    // The memory will be automatically freed when ptr goes out of scope.
    // Rust does not allow double-freeing or dereferencing a freed pointer.
}