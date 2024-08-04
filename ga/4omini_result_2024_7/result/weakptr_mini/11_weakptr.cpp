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
    // Create shared pointer using Rc
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));

    // Create a weak pointer from the shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr);

    // Attempt to obtain a shared reference from the weak pointer
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify the data
        let value = &shared_ptr.value; // Extracting the field value
        println!("Value: {}", value);
    }
}