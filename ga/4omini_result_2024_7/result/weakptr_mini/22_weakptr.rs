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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer
    
    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain Rc
        let mut value = shared_ptr.value; // dereference to get the value
        value = 20; // modify value
        println!("Value: {}", value);
    }
}