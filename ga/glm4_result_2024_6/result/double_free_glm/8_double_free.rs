use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::new::<i32>();
    let ptr = unsafe { alloc(layout) };
    unsafe { ptr::write(ptr, 10) };
    unsafe { dealloc(ptr, layout) };
    // The second call to free() is UB in Rust
    // printf!("%d", *ptr); // UB
}