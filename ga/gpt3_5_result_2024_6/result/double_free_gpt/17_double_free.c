use std::ptr;

fn main() {
    let ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);
    unsafe {
        let boxed_ptr = Box::from_raw(raw_ptr);
        drop(boxed_ptr);
    }
}