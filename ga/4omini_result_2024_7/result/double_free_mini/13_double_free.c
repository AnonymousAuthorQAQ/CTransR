fn main() {
    // Allocate memory for an integer
    let ptr: Box<i32> = Box::new(10);

    // Since Rust automatically manages memory, we don't need to free `ptr`
    // The memory will be freed automatically when `ptr` goes out of scope

    // Print the value
    println!("{}", *ptr);
}

// Note: If we wanted to explicitly manage the memory, we could use `std::ptr`, 
// but it's rare to do so in idiomatic Rust.