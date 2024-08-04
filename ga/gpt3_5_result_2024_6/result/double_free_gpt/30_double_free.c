use std::ptr;
use std::mem;

fn main() {
    let ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    unsafe {
        ptr::drop_in_place(raw_ptr);
        mem::drop(Box::from_raw(raw_ptr));
    }
    
    // Trying to access the value after freeing memory will lead to a compile-time error
    // println!("{}", *Box::from_raw(raw_ptr)); 

    println!("{}", raw_ptr); // This will print a memory address
}