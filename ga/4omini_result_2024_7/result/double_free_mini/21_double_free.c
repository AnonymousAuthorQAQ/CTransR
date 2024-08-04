fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);

    // Print the value
    println!("{}", *ptr);

    // Memory will be automatically freed when ptr goes out of scope
    // No need for an explicit free call as in C
}