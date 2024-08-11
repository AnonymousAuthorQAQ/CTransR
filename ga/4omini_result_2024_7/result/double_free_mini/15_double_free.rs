fn main() {
    // Allocate memory for an integer and set its value to 10
    let ptr = Box::new(10);

    // Print the value
    println!("{}", *ptr);
    
    // No need to manually free memory, Box will automatically deallocate when it goes out of scope
}