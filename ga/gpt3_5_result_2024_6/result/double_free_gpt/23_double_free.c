fn main() {
    let ptr = Box::new(10);
    // Memory is automatically released when ptr goes out of scope
    println!("{}", *ptr); // This will print the value stored in the box
}