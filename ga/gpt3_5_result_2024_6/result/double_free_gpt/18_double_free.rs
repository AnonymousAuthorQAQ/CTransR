fn main() {
    let mut ptr = Box::new(10);
    // Memory gets automatically deallocated when `ptr` goes out of scope
    // No need to manually call free in Rust
    // free(ptr);
    // free(ptr);
    println!("{}", *ptr); // This will not compile, as it violates borrowing rules in Rust
}