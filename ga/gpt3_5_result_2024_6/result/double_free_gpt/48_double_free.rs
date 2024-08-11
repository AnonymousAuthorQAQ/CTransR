use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    ptr = Box::new(20); // Reassigning the Box pointer instead of calling free

    println!("{}", *ptr);
}