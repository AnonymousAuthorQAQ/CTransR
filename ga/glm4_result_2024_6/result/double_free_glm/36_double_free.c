use std::ptr::null_mut;

fn main() {
    let mut ptr: *mut i32 = null_mut();
    unsafe {
        ptr = std::malloc(std::mem::size_of::<i32>()) as *mut i32;
        if !ptr.is_null() {
            *ptr = 10;
            std::free(ptr);
            std::free(ptr); // This is undefined behavior
            println!("{}", *ptr); // This is undefined behavior
        }
    }
}