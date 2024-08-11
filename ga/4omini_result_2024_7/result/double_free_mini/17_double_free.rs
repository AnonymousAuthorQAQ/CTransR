fn main() {
    let ptr = Box::new(10); // Allocate an integer on the heap and initialize it to 10
    // No need to explicitly free the memory, Box will take care of it
    // However, we will not attempt to free it again as in the C code
    
    // To print the value, we need to dereference the Box
    println!("{}", *ptr);
    
    // If we want to explicitly drop the Box before the end of the scope
    // we can do so like this:
    // drop(ptr);
    
    // Note: Attempting to dereference ptr after it has been dropped will cause a compile-time error
}