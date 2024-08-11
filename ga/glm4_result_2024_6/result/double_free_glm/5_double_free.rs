use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    unsafe {
        ptr::drop_in_place(ptr);
        ptr::drop_in_place(ptr);
    }
    println!("{}", *ptr);
}
use std::mem;

fn main() {
    let mut x = Box::new(10);
    println!("{}", *x);
    mem::drop(x);
    // x is now freed and cannot be used again
}