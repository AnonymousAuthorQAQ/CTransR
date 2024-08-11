fn main() {
    // Allocate memory on the heap for a single i32 and assign its value to 10
    let ptr = Box::new(10);

    // Attempting to access the value after "freeing" (which is not done in Rust as in C)
    let value = *ptr; // Safely dereference the Box

    // Print the value; in Rust, we don't have a free call since the Box will be dropped at end of scope
    println!("{}", value);
}