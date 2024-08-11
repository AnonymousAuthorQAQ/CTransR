fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);
    
    // Print the value before dropping the box
    println!("{}", *ptr);
    
    // The Box will automatically be freed when it goes out of scope
}