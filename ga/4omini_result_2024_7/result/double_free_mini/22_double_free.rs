fn main() {
    // Allocate a box to hold an integer
    let ptr = Box::new(10);
    
    // Here we can use *ptr just normally, there's no need to manually free it.
    println!("{}", *ptr);
    
    // ptr will be automatically dropped when it goes out of scope
}