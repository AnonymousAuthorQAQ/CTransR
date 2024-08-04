fn main() {
    // Allocate memory on the heap for an integer
    let ptr: Box<i32> = Box::new(10);
    
    // Attempting to access the value of ptr after it goes out of scope
    // This is safe in Rust - we can't double free because of ownership rules.
    // Let's print the value before it goes out of scope.
    println!("{}", *ptr); // This will print the value
    
    // `ptr` goes out of scope here and memory will be automatically freed.
    // There's no way to call free twice for the same memory in Rust, 
    // so we won't replicate that error.
}