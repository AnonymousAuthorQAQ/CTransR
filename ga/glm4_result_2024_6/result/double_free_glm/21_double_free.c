fn main() {
    let mut ptr = Box::new(10);
    // Rust automatically frees the memory when the Box goes out of scope
    // Therefore, we do not need to explicitly free the memory as in C
    println!("{}", *ptr);
}