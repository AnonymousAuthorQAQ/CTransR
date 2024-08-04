fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10); // Automatically handles memory allocation and deallocation

    // Print the value
    println!("{}", *ptr); // Access the value safely

    // Manually deallocate if needed (Box will do this automatically when it goes out of scope)
    // Rust will automatically free memory when `ptr` goes out of scope as it is a Box.
}