use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);
    unsafe {
        Box::from_raw(raw_ptr);
        // Box::from_raw(raw_ptr); // This will cause a double free error
    }
}