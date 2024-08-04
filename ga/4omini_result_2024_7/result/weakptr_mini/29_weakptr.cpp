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
    // Create an `Arc` (similar to `std::shared_ptr` in C++)
    let arc_ptr = Arc::new(MyStruct::new(10));
    
    // Create a `Weak` pointer (similar to `std::weak_ptr` in C++)
    let weak_ptr = Arc::downgrade(&arc_ptr);
    
    // Try to upgrade the `Weak` pointer to an `Arc`
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify data
        shared_ptr.value = 20; 
        println!("Value: {}", shared_ptr.value);
    } else {
        println!("Weak pointer is expired");
    }
}