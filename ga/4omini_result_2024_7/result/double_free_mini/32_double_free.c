fn main() {
    let ptr = Box::new(10); // Allocate an integer on the heap
    // We don't explicitly free memory in Rust; it will be automatically released when `ptr` goes out of scope
    println!("{}", *ptr); // Dereference the Box to get the value

    // In Rust, we don't call free, and double freeing is not an issue since ownership ensures safety.
}