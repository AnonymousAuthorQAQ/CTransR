fn main() {
    // Allocating space for an integer
    let ptr = Box::new(10); // Using Box to allocate memory on the heap
    println!("{}", *ptr); // Print the value stored in the box

    // The memory will be automatically freed when ptr goes out of scope
}