fn main() {
    // Allocating memory dynamically for an integer
    let ptr = Box::new(10);

    // Printing the value of the allocated integer
    println!("{}", ptr);

    // In Rust, boxes are automatically freed when they go out of scope,
    // but you can manually drop a box as follows:
    drop(ptr);
    
    // Now, we cannot use `ptr` anymore after it’s dropped as Rust's borrow checker prevents it.
    // Any attempt to use `ptr` after it's dropped will result in a compile error.
    // To illustrate, if we uncomment the next line, it will cause a compile-time error.
    // println!("{}", ptr); // This line would not compile
    
    // Instead, let's explicitly create a new Box after dropping the old one.
    let ptr = Box::new(10); // Allocating memory again
    println!("{}", ptr); // Safe to use as it's a fresh Box
}