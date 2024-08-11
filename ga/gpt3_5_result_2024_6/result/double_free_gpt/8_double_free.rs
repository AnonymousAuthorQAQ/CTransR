use std::ptr;

fn main() {
    let ptr = Box::new(10);
    drop(ptr);
    drop(ptr);
    println!("{}", ptr); // will give an error because ptr has been dropped
}