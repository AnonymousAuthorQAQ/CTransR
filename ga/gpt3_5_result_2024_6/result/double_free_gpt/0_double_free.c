use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    ptr.as_mut();
    println!("{}", *ptr);
    // "free" is not used in Rust, as memory is managed automatically
}