use std::ptr;

fn main() {
    let mut ptr: *mut i32 = ptr::null_mut();
    unsafe {
        ptr = Box::into_raw(Box::new(10));
        drop(Box::from_raw(ptr));
        // This line will cause a panic because the memory has already been freed
        // drop(Box::from_raw(ptr));
        println!("{}", *ptr);
    }
}