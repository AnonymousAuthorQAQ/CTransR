fn main() {
    // Allocating memory for an i32
    let ptr = Box::new(10); // `Box` handles memory allocation and deallocation safely
    
    // The value can be accessed using the dereference operator
    println!("{}", *ptr); // This prints the value stored in the box (10)

    // Memory is automatically freed when `ptr` goes out of scope
}