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
    let ptr = Arc::new(MyStruct::new(10)); // Create shared pointer
    let weak_ptr = Arc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain Arc
        let mut shared_ref = Arc::get_mut(&mut shared_ptr).unwrap();
        shared_ref.value = 20; // modify data
        println!("Value: {}", shared_ref.value);
    }
}