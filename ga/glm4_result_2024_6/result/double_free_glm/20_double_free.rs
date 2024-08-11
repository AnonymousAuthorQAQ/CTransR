fn main() {
    let mut ptr = Box::new(10);
    *ptr = 10;
    drop(ptr);
    // The following line is a double free error in Rust and is therefore commented out
    // drop(ptr);
    println!("{}", *ptr);
}