use std::ptr;

fn main() {
    let mut ptr = ptr::null_mut();
    ptr = Box::into_raw(Box::new(10));
    unsafe {
        let _ = Box::from_raw(ptr);
        let _ = Box::from_raw(ptr);
    }
    // The following line would cause undefined behavior, as the memory has already been freed
    // println!("{}", *ptr);
}