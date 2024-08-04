use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::new::<i32>();
    let ptr = unsafe { alloc(layout) };
    if !ptr.is_null() {
        unsafe {
            *(ptr as *mut i32) = 10;
            dealloc(ptr, layout);
            dealloc(ptr, layout); // This line will cause a double free error at runtime
            println!("{}", *(ptr as *mut i32)); // This line will cause a dangling pointer error at runtime
        }
    }
}