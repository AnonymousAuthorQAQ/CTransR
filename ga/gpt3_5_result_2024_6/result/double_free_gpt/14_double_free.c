fn main() {
    let ptr = Box::new(10);
    // Memory allocated using Box::new, no need for manual deallocation

    println!("{}", *ptr);
    // Access and print the value stored in the pointer
}