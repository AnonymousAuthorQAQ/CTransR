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
    let ptr = Arc::new(MyStruct::new(10)); // Create Arc
    let weak_ptr = Arc::downgrade(&ptr); // Create Weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to obtain Arc
        let mut value = &shared_ptr.value; // Accessing the value
        println!("Value: {}", value); // Print current value
        
        // Modify data (values contained in inner mutable reference)
        let mut mutable_shared_ptr = Arc::get_mut(&mut Arc::clone(&shared_ptr)).unwrap();
        mutable_shared_ptr.value = 20; // Modify value

        println!("Updated Value: {}", mutable_shared_ptr.value); // Print updated value
    }
}