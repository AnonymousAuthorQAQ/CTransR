fn main() {
    let ptr: Box<i32> = Box::new(10); // Allocate memory for an integer and initialize it to 10
    println!("{}", *ptr); // Print the value

    // In Rust, we do not explicitly free the memory; it is automatically freed when ptr goes out of scope.
}