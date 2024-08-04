use std::ptr::null_mut;

fn main() {
    let mut ptr: *mut i32 = null_mut();
    unsafe {
        ptr = Box::into_raw(Box::new(10));
        // The first free is correct
        drop(unsafe { Box::from_raw(ptr) });
        // The second free is incorrect and will cause undefined behavior
        // drop(unsafe { Box::from_raw(ptr) });
    }
    // Accessing the already freed memory would also cause undefined behavior
    // println!("{}", unsafe { *ptr });
}