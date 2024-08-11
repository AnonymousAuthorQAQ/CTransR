use std::ptr;
use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    
    let raw_ptr = Box::into_raw(ptr);
    unsafe {
        ptr::drop_in_place(raw_ptr);
    }
    
    // Attempting to access the value stored in the raw pointer
    let value = unsafe { *raw_ptr };
    println!("{}", value);
}