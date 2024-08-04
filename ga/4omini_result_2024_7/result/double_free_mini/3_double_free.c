fn main() {
    let ptr: Box<i32> = Box::new(10); // Allocate memory for an integer and initialize it to 10
    // The memory will be automatically freed when 'ptr' goes out of scope
    println!("{}", *ptr); // Dereference and print the value
}