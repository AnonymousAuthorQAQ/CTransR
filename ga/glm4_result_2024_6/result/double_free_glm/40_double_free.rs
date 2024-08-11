fn main() {
    let mut ptr = Box::new(10);
    println!("{}", *ptr);
    // The memory is automatically deallocated when ptr goes out of scope.
}