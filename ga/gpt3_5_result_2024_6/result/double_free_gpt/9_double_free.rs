use std::ptr;

fn main() {
    let ptr = &mut Box::new(10);
    let raw_ptr = &mut **ptr as *mut i32;
    unsafe {
        ptr::drop_in_place(raw_ptr);
    }
    println!("{}", raw_ptr); // This will lead to a use-after-free error
}