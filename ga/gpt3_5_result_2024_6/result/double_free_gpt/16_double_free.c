fn main() {
    let mut ptr = Box::new(10);
    // 'ptr' is automatically deallocated when it goes out of scope
    // No need to manually free memory in Rust

    // Trying to access 'ptr' after it has been deallocated will result in a compile-time error
    // Uncomment the line below to see the error
    // println!("{}", *ptr);
}