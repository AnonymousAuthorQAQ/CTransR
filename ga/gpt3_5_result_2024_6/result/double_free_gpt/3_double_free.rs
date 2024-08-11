use std::ptr;
use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    unsafe {
        ptr::drop_in_place(raw_ptr);
        mem::forget(raw_ptr); // Prevent double-free by telling Rust to forget about this raw pointer
    }
}