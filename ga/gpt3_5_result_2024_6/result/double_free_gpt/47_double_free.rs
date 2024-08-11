use std::mem;

fn main() {
    let mut ptr = Box::new(10);
    *ptr = 10;
    mem::drop(ptr);
    //mem::drop(ptr); // can't drop twice in Rust
    // println!("{}", *ptr); // cannot use dereference after dropping
}