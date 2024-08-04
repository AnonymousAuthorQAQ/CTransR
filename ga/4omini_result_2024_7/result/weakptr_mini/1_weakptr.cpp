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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create shared pointer (Rc)
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain Rc
        // Modify data
        shared_ptr.value = 20; 
        println!("Value: {}", shared_ptr.value);
    }
    // The destructor will be called when ptr goes out of scope
}