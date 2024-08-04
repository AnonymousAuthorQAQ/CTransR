use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    unsafe {
        std::ptr::drop_in_place(ptr.as_mut());
        std::ptr::drop_in_place(ptr.as_mut());
    }
    println!("{}", *ptr);
}