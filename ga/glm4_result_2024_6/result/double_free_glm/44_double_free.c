fn main() {
    let mut ptr = Box::new(10); // Allocate memory on the heap for an integer and initialize it to 10
    println!("{}", *ptr); // Print the value of the integer

    // ptr is automatically dropped and memory is deallocated here
}