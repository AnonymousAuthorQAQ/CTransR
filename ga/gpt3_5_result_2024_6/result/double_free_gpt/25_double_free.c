use std::ptr;
use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    unsafe {
        let ptr = &mut *raw_ptr;
        println!("{}", *ptr);
        drop(Box::from_raw(raw_ptr));
    }
}