fn main() {
    let mut ptr = Box::new(10);
    *ptr = 10;
    drop(ptr);
    // ptr is already dropped, so the next line is commented out to avoid a double-free error
    // drop(ptr);
    // Attempting to print the value pointed to by ptr would result in an error
    // println!("{}", *ptr);
}