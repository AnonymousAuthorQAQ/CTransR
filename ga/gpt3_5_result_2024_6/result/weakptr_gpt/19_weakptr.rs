use std::sync::{Arc, Weak};
use std::rc::Rc;

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
    let my_struct = Arc::new(MyStruct::new(10));
    let weak_ptr = Arc::downgrade(&my_struct); // Create weak pointer
    
    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain shared_ptr
        shared_ptr.value = 20; // modify data
        println!("Value: {}", shared_ptr.value);
    }
}