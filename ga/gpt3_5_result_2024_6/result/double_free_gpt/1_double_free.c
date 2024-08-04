fn main() {
    let mut ptr = Box::new(10);
    // Dereferencing the pointer to access the value
    println!("{}", *ptr);
    // Dropping the value, (equivalent to freeing memory in C)
    drop(ptr);
    // Trying to access the value of the integer pointer after it has been freed
    // println!("{}", *ptr);
}