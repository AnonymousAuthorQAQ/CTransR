use std::alloc::{alloc, dealloc};
use std::ptr;

fn main() {
    let ptr = unsafe { alloc(Layout::from_size_align_unchecked(4, 4)) };
    unsafe { *ptr = 10 };
    unsafe { dealloc(ptr, Layout::from_size_align_unchecked(4, 4)) };
    // The second call to dealloc is a double free and should be removed
    // printf! is not available in Rust, so we use println! instead
    println!("{}", unsafe { *ptr });
    // However, trying to print *ptr after it has been freed will result in undefined behavior
}