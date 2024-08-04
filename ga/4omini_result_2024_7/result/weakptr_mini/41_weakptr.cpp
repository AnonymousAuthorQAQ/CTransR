use std::rc::{Rc, Weak};

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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Shared pointer equivalent
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr);  // Weak pointer equivalent

    // Attempt to obtain Rc from Weak and modify value
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        let mut value = shared_ptr.value; // Create mutable reference to value
        value = 20; // Modify the data
        println!("Value: {}", value);
    }
}