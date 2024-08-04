use std::ptr;

fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);
    unsafe {
        ptr::drop_in_place(raw_ptr);
    }
    println!("{}", unsafe { *raw_ptr });
}