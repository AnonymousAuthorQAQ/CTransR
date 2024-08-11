fn main() {
    let mut ptr = Box::new(10);
    println!("{}", *ptr);
    // Rust automatically drops the Box at the end of the block, so there's no need to explicitly free it.
}