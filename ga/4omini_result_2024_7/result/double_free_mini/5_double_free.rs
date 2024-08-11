fn main() {
    // Allocate an integer on the heap
    let ptr = Box::new(10); // Box handles memory automatically
    // No need to free manually, Box will clean up when it goes out of scope

    // Print the value stored in the Box
    println!("{}", *ptr); // Dereference the Box to get the value
}