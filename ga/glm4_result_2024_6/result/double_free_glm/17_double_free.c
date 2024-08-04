fn main() {
    let mut ptr: *mut i32 = unsafe { std::alloc::alloc(std::alloc::Layout::new::<i32>()) };
    if !ptr.is_null() {
        unsafe { *ptr = 10 };
        println!("{}", unsafe { *ptr });
        unsafe { std::alloc::dealloc(ptr, std::alloc::Layout::new::<i32>()) };
        ptr = std::ptr::null_mut();
    }
}