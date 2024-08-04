fn main() {
    // Allocate memory for an integer on the heap
    let ptr = Box::new(10);

    // Print the value pointed to by ptr
    println!("{}", *ptr);

    // The memory will be automatically freed when ptr goes out of scope
}