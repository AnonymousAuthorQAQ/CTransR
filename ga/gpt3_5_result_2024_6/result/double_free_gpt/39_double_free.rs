use std::ptr;

fn main() {
    let ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);
    unsafe {
        ptr::drop_in_place(raw_ptr);
        ptr::drop_in_place(raw_ptr); // Attempting to drop the memory again
        println!("{}", *raw_ptr); // Accessing the value at the memory address
    }
}