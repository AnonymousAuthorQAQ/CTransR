use std::sync::{Arc, Weak};

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> Self {
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
    let ptr = Arc::new(MyStruct::new(10));    // Create an Arc (similar to shared_ptr)
    let weak_ptr = Arc::downgrade(&ptr);      // Create a Weak pointer

    // Attempt to obtain a strong reference using a temporary Arc
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        shared_ptr.value = 20; // Modify data
        println!("Value: {}", shared_ptr.value);
    }
}