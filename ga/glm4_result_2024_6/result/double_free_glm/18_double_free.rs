fn main() {
    let mut ptr = Box::new(10);
    println!("{}", *ptr);
    // The following line is commented out because it would cause a panic.
    // *ptr = 20;
    // println!("{}", *ptr);
}