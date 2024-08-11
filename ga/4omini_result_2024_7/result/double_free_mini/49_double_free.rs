fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);
    
    // Print the value
    println!("{}", *ptr);
    
    // The Box will automatically be freed when it goes out of scope
    // There's no need to manually free memory like in C.
}