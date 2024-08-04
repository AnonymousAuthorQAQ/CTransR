fn main() {
    let mut ptr: Option<Box<i32>> = Some(Box::new(10)); // Allocate memory and initialize to 10

    // Simulate freeing the pointer
    ptr = None; // "Freeing" the pointer by setting it to None

    // Attempting to dereference after the "free"
    // This will be a safe operation, but it will panic since `ptr` is None
    match &ptr {
        Some(value) => println!("{}", value),
        None => println!("Pointer has been freed or is invalid."),
    }
}