fn main() {
    let ptr = Box::new(10);
    drop(ptr);
    // Trying to access ptr here would result in a compile-time error
    // because Rust does not allow accessing or deallocating memory
    // that has already been deallocated.
}