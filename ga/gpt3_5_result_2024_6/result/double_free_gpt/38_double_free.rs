fn main() {
    let mut ptr = Box::new(10);
    *ptr = 10;
    drop(ptr);
    // Trying to access the memory after dropping it may result in a compilation error
    // println!("{}", *ptr); 
}