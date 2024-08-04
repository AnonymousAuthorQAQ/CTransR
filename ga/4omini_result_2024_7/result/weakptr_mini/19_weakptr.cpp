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
    let ptr = Rc::new(MyStruct::new(10)); // Create an Rc
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create a Weak pointer

    // Attempt to obtain a strong reference from the weak pointer
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        shared_ptr.value = 20; // Modify data
        println!("Value: {}", shared_ptr.value);
    }
}