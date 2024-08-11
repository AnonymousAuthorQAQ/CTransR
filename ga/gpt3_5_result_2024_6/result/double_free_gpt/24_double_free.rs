fn main() {
    let mut ptr = Box::new(10);
    // Memory deallocated automatically when ptr goes out of scope
    // No need to explicitly call free as in C
    println!("{}", *ptr);
}