fn main() {
    let mut ptr = Box::new(10);
    *ptr = 10;
    drop(ptr);
    // Attempting to access `ptr` here would result in a compile-time error
}