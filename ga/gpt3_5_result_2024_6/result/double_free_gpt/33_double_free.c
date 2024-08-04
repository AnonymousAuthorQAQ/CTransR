use std::ptr;
use std::mem;

fn main() {
    let ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    unsafe {
        let borrowed_ptr = &*raw_ptr;
        println!("{}", borrowed_ptr);

        // Freeing the memory - not recommended in Rust
        drop(Box::from_raw(raw_ptr));

        // Trying to access the memory after freeing
        // This would result in a compiler error in Rust
        // println!("{}", borrowed_ptr);
    }
}