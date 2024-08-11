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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer
    
    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain strong pointer
        let mut shared_ptr = Rc::make_mut(&mut shared_ptr); // Get a mutable reference
        shared_ptr.value = 20; // modify data
        println!("Value: {}", shared_ptr.value);
    }
}