fn main() {
    let ptr = Box::new(10);
    drop(ptr);
    //drop(ptr); // Attempting to drop the pointer a second time will result in a compilation error
    //println!("{}", *ptr); // Trying to access the value after dropping the pointer will result in a compilation error
}