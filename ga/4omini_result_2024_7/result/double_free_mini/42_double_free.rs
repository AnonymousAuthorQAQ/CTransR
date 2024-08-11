fn main() {
    // Allocate memory for an integer and assign a value
    let mut ptr: Box<i32> = Box::new(10);

    // Print the value before freeing
    println!("{}", *ptr);

    // Free the memory (Box will automatically free when it goes out of scope)
    drop(ptr);

    // After dropping, we can't safely access the value anymore
    // Uncommenting the following line would cause a compile-time error
    // println!("{}", *ptr);
}