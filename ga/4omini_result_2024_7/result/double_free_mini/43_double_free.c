fn main() {
    // Allocate memory for an integer on the heap
    let ptr = Box::new(10); // Box allocates memory on the heap and ensures it's freed when it goes out of scope

    // Print the value stored in the Box
    println!("{}", *ptr); // Dereference the Box to get the value

    // The Box will be automatically freed when it goes out of scope
}