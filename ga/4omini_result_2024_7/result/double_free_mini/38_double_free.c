fn main() {
    // Allocate memory for an integer on the heap
    let ptr = Box::new(10); // Create a Box containing the value 10

    //	ptr is a smart pointer; it automatically deallocates when it goes out of scope
    // Let's print the value stored in `ptr`
    println!("{}", *ptr);
} // `ptr` is automatically freed when it goes out of scope