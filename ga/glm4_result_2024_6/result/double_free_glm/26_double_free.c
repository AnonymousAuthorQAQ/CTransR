use std::ptr::null_mut;

fn main() {
    let mut ptr: *mut i32 = null_mut();
    unsafe {
        ptr = Box::into_raw(Box::new(10));
        Box::from_raw(ptr);
        // The second free() is intentionally commented out to avoid double free error
        // Box::from_raw(ptr);
        println!("{}", *ptr);
    }
}