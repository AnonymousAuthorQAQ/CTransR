fn main() {
    let mut boxed_int = Box::new(10);
    println!("Boxed int: {}", boxed_int);

    // Rust automatically drops the Box when it goes out of scope
    // This is the equivalent of calling free in C
    drop(boxed_int);

    // boxed_int is now a dangling pointer, but Rust prevents us from using it
    // Attempting to use *boxed_int would result in a compile-time error

    // Rust does not have an equivalent to printf that prints a raw pointer
    // So we cannot directly print the value of the dangling pointer
    // However, we can print the address of the pointer
    println!("Address of dangling pointer: {:p}", &boxed_int);

    // Rust prevents double-free by setting the pointer to None after drop
    // So we cannot drop it again
    // drop(boxed_int); // This line would cause a compile-time error
}