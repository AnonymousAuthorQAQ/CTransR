fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);
    
    // Print the value before freeing
    println!("{}", *ptr);

    // Freeing the memory (Box will automatically be dropped here)
    drop(ptr);

    // Attempting to access the value after freeing
    // Uncommenting the next line would lead to a compile-time error
    // println!("{}", *ptr); // This line is commented out to prevent undefined behavior
}