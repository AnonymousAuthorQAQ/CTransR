fn main() {
    // Allocate an integer on the heap
    let ptr = Box::new(10);

    // Print the value
    println!("{}", *ptr);

    // No need to manually free the memory, it will be automatically freed when `ptr` goes out of scope
}