fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    // Freeing the memory twice which is explicitly done in C but will panic in Rust
    unsafe {
        // Equivalent of double free and demonstrates that freed memory cannot be accessed
        Box::from_raw(raw_ptr);
        Box::from_raw(raw_ptr); // Panics
        println!("{}", *raw_ptr); // Dead code, as it panics before reaching here
    }
}