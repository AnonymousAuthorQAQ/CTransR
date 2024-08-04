fn main() {
    let mut ptr = Box::new(10); // Allocate memory for an integer and assign value 10
    drop(ptr); // Free the allocated memory
    drop(ptr); // Attempt to free the memory again, which will cause a panic
    // Trying to access the value stored at the memory location pointed by ptr will result in a panic
}