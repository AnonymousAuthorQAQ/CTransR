use std::rc::{Rc, Weak};

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
    // Create a strong reference to MyStruct
    let ptr = Rc::new(MyStruct::new(10));
    
    // Create a weak reference to MyStruct
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr);
    
    // Attempt to obtain a strong reference
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        shared_ptr.value = 20; // modify data
        println!("Value: {}", shared_ptr.value);
    } else {
        println!("Weak pointer is no longer valid.");
    }
}