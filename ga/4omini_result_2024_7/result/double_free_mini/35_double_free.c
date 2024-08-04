fn main() {
    // Allocate memory for an integer
    let ptr = Box::new(10);

    // Print the value
    println!("{}", *ptr);

    // The Box will be automatically deallocated when it goes out of scope
}