use std::ptr;

fn main() {
    let mut ptr = ptr::null_mut();
    unsafe {
        ptr = Box::into_raw(Box::new(10));
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
    // The pointer has already been freed, so attempting to print the value would cause undefined behavior.
    // Instead, we print a placeholder value to indicate that the pointer has been freed.
    println!("Placeholder");
}