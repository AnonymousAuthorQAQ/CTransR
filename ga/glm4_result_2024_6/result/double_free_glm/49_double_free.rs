fn main() {
    let mut ptr = Box::new(10);
    println!("{}", *ptr);
    // Rust automatically deallocates the memory when the Box goes out of scope
}