use std::ptr;
use std::alloc::{alloc, dealloc};

fn main() {
    let mut ptr = unsafe { alloc(Layout::new::<i32>()) };
    unsafe {
        *ptr = 10;
        dealloc(ptr, Layout::new::<i32>());
        dealloc(ptr, Layout::new::<i32>()); // This line will cause a double free error
        println!("{}", *ptr); // This line will cause a dangling pointer error
    }
}