fn main() {
    let mut ptr = Box::new(10);
    let raw_ptr = Box::into_raw(ptr);

    unsafe {
        Box::from_raw(raw_ptr);
        Box::from_raw(raw_ptr); // Attempting to free the memory pointer again
        println!("{}", *raw_ptr); // Accessing the value of the pointer after freeing
    }
}