fn main() {
    // Allocate an integer on the heap using Box
    let ptr = Box::new(10);
    
    // Dereference the Box to get the value
    println!("{}", *ptr); // This will print the value before dropping

    // As Box is dropped automatically here at the end of the scope,
    // we are not going to manually "free" it like in C. 
    // No double free or dereference afterwards here will happen.
    
    // Normal Rust behavior ensures safety, so attempting to use a dropped
    // pointer would lead to a compile-time error, unlike in C.
}