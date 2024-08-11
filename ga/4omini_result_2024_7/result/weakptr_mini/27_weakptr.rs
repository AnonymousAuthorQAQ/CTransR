use std::sync::{Arc, Weak};

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> MyStruct {
        println!("Constructor");
        MyStruct { value: v }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let ptr = Arc::new(MyStruct::new(10)); // Create an Arc for shared ownership
    let weak_ptr = Arc::downgrade(&ptr); // Create a Weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to upgrade to Arc
        shared_ptr.value = 20; // Modify data
        println!("Value: {}", shared_ptr.value);
    }
}