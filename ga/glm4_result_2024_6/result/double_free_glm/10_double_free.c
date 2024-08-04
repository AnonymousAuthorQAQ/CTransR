use std::ptr;
use std::alloc::{alloc, dealloc};

fn main() {
    unsafe {
        let mut ptr = alloc(Layout::from_size_align_unchecked(4, 4));
        ptr.write(10);
        dealloc(ptr, Layout::from_size_align_unchecked(4, 4));
        // This line will cause a double free error, but it is commented out for demonstration.
        // dealloc(ptr, Layout::from_size_align_unchecked(4, 4));
    }
    // This line will cause a use after free error, but it is commented out for demonstration.
    // println!("{}", unsafe { *ptr });
}