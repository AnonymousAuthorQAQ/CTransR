fn main() {
    let mut ptr = Box::new(10);
    println!("{}", *ptr);
    // In Rust, memory is automatically deallocated when the variable goes out of scope
}