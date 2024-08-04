fn main() {
    // Allocate memory for an integer using Box (similar to malloc in C)
    let ptr = Box::new(10); // Allocating memory and assigning the value 10

    // Printing the value
    println!("{}", *ptr); // Dereferencing ptr to get the value

    // Box automatically deallocates memory when it goes out of scope
    // No need to explicitly free it like in C
}