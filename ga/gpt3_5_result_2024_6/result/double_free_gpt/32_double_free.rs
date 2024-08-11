use std::ptr;
use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    unsafe {
        Box::from_raw(raw_ptr);
        // Double free is avoided in Rust
        // Box::from_raw(raw_ptr);
    }
    
    // Accessing `raw_ptr` after freeing it using Box::from_raw will lead to undefined behavior
    // println!("{}", *ptr);
}