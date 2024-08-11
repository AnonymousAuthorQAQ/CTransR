use std::ptr::null_mut;

fn main() {
    let mut ptr: *mut i32 = null_mut();
    unsafe {
        ptr = Box::into_raw(Box::new(10));
        std::boxed::Box::from_raw(ptr);
        std::boxed::Box::from_raw(ptr);
    }
    unsafe {
        println!("{}", *ptr);
    }
}