use std::ptr::null_mut;

fn main() {
    let mut ptr: *mut i32 = null_mut();
    ptr = Box::into_raw(Box::new(10));
    unsafe {
        Box::from_raw(ptr);
    }
    // This line would cause a double free error, but it is commented out
    // Box::from_raw(ptr);

    // Since the pointer has been freed, dereferencing it would lead to undefined behavior
    // println!("{}", *ptr);
}