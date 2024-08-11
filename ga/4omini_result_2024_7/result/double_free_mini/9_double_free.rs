fn main() {
    // Allocate memory on the heap for an integer
    let ptr = Box::new(10); // Equivalent to malloc and initialization

    // Print the value before it is freed
    println!("{}", ptr);
    
    // Free the memory explicitly (Box is dropped)
    drop(ptr);
    
    // The below print has been commented out to avoid undefined behavior
    // The value is not valid anymore since we have dropped the box.
    // println!("{}", ptr); // This would lead to a compile-time error

    // If you want to see memory handling, we can just demonstrate it
    // without accessing freed memory
}