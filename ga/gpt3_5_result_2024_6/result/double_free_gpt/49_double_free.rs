use std::ptr;

fn main() {
    let mut ptr: *mut i32 = Box::into_raw(Box::new(10));
    // Free memory
    unsafe {
        drop(Box::from_raw(ptr));
    }
    // This will cause a dangling pointer error if uncommented
    // drop(Box::from_raw(ptr));
    println!("{}", unsafe { *ptr });
}