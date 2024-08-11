use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    unsafe {
        std::ptr::drop_in_place(&mut ptr);
        std::ptr::drop_in_place(&mut ptr);
    }
    println!("{}", *ptr);
}