use std::ptr;
use std::mem;

fn main() {
    let mut ptr: *mut i32 = unsafe { mem::transmute(Box::new(10)) };
    unsafe {
        ptr::drop_in_place(ptr);
        ptr::drop_in_place(ptr); // explicitly drop the pointer twice
        println!("{}", *ptr); // try to access the value after freeing
    }
}