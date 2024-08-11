use std::ptr;
use std::mem;

fn main() {
    let ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    // Free the memory - equivalent to free(ptr) in C
    unsafe { 
        Box::from_raw(raw_ptr);
        mem::forget(raw_ptr); // This simulates double-free as in the C code
    }

    // Try to access the memory location
    // This will result in undefined behavior similar to the C code
    // Uncomment the line below to see the segmentation fault
    // println!("{}", *raw_ptr);
}