fn main() {
    // Allocate memory for an integer
    let ptr: *mut i32 = Box::into_raw(Box::new(10)); // Allocating and converting to a raw pointer

    unsafe {
        // Attempting to access the value before freeing
        println!("{}", *ptr); // This will print `10`

        // Free the memory (Box::from_raw takes ownership of the raw pointer and frees it)
        let _ = Box::from_raw(ptr);

        // The following line simulates the erroneous double free or dereferencing after free.
        // Uncommenting this will lead to a runtime error (use after free).
        // println!("{}", *ptr); // Uncommenting this will cause a panic.
    }
}