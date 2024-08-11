fn main() {
    let ptr = Box::new(10); // Allocating memory for an integer on the heap
    println!("{}", *ptr); // Printing the value
    // No need to manually free the memory, Box will automatically handle it.
}