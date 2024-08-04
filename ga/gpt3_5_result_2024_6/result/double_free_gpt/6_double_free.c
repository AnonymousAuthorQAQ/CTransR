use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    ptr::drop(ptr.as_mut());
    ptr::drop(ptr.as_mut());
    println!("{}", *ptr);
}