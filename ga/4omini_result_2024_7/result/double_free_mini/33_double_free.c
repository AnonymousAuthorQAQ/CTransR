fn main() {
    // Allocate memory for an integer and initialize it with the value 10
    let mut value: Box<i32> = Box::new(10);

    // Print the value before freeing
    println!("{}", value);

    // Freeing the memory is handled automatically when `value` goes out of scope
    // If you want to demonstrate free-like behavior, you can drop it explicitly
    drop(value);

    // After dropping, accessing `value` would be unsafe and is not allowed
    // Uncommenting the next line will trigger a compile error: 
    // println!("{}", value);
}

// Note: The above code ignores the behavior of trying to access freed memory,
// because in Rust, once you drop a box, you cannot access it anymore.
// This is a safety feature in Rust to prevent undefined behavior.