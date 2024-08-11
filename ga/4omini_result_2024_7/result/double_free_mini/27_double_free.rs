fn main() {
    // Allocate memory for an integer on the heap
    let ptr: Box<i32> = Box::new(10);

    // Attempt to "free" the memory (Rust automatically handles this when `ptr` goes out of scope)
    drop(ptr); // This simulates the `free` in C

    // Uncommenting the next line will cause a compilation error
    // let val = *ptr; // Trying to access the value after it's dropped will not compile

    // Instead, we can use an option type to mimic the double-free scenario
    let mut opt_ptr: Option<Box<i32>> = Some(Box::new(10));
    opt_ptr = None; // Drop the value, similar to freeing memory

    // The following line would also cause a compilation error
    // let val = *opt_ptr.unwrap(); // Accessing after dropping would fail

    // To illustrate usage, you can safely use the value before dropping
    if let Some(value) = opt_ptr {
        println!("{}", value); // Safe to print before dropping
    } else {
        println!("Pointer is dropped, cannot access value");
    }
}