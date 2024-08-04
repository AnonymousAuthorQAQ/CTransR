use std::sync::{Arc, Weak};

struct MyStruct {
    value: i32,
}

impl MyStruct {
    // Constructor
    fn new(v: i32) -> Self {
        println!("Constructor");
        MyStruct { value: v }
    }
}

// Drop trait to provide destructor behavior
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    // Create an Arc (equivalent to std::shared_ptr)
    let ptr = Arc::new(MyStruct::new(10));
    
    // Create a Weak reference (equivalent to std::weak_ptr)
    let weak_ptr = Arc::downgrade(&ptr);

    // Attempt to obtain an Arc from Weak
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify data
        shared_ptr.value = 20;
        println!("Value: {}", shared_ptr.value);
    } else {
        println!("Weak pointer is expired.");
    }
}