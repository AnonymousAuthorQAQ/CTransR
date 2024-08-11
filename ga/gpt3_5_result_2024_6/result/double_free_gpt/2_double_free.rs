fn main() {
    let mut ptr = Box::new(10);
    // Memory is automatically deallocated when the Box goes out of scope
    println!("{}", *ptr);
}