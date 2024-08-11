use std::ptr;
use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);
    unsafe {
        let ptr = mem::transmute::<*mut i32, *mut i32>(raw_ptr);
        ptr::drop_in_place(ptr);
    }
}