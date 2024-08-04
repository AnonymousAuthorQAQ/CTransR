fn main() {
    let mut ptr = Box::new(10);
    println!("{}", *ptr);
    // ptr is automatically dropped at the end of the scope
}