fn main() {
    let mut ptr = Box::new(10);
    *ptr = 10;
    drop(ptr);
    // Trying to access the memory location here would result in a compile-time error
}