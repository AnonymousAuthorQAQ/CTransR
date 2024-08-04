fn main() {
    let ptr = Box::new(10);
    drop(ptr);
    // Attempting to access the value pointed to by ptr here would result in a compile-time error
}