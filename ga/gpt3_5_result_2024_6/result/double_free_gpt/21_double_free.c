fn main() {
    let ptr = Box::new(10);
    // ptr is automatically deallocated when it goes out of scope, no need to manually free it
    println!("{}", *ptr);
}