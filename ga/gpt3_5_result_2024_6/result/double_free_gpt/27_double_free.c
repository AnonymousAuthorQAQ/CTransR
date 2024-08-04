use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    ptr.as_mut_ptr();
    drop(ptr);
    drop(ptr);
    println!("{}", *ptr);
}