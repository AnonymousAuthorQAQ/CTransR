use std::ptr::null_mut;

fn main() {
    let mut ptr: *mut i32 = null_mut();
    ptr = Box::into_raw(Box::new(10));
    unsafe {
        drop(Box::from_raw(ptr));
        // This line will cause an error since the pointer has already been freed
        // drop(Box::from_raw(ptr));
    }
    // This line will cause an error since the pointer has already been freed
    // println!("{}", *ptr);
}