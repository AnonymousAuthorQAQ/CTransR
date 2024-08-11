use std::ptr;
use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    
    // Dropping the Box will free the memory
    let raw_ptr = Box::into_raw(ptr);
    
    // Do not free the memory again as Rust will handle it
    // mem::drop(ptr);

    // Dereference the raw pointer to access the value
    let value = unsafe { *raw_ptr };
    
    println!("{}", value);
    
    // Deallocate the memory manually
    unsafe {
        Box::from_raw(raw_ptr);
    }
}