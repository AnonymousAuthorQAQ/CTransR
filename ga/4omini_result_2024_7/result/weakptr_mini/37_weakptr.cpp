use std::cell::RefCell;
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
    let ptr = Rc::new(RefCell::new(MyStruct::new(10))); // Create a shared pointer
    let weak_ptr: Weak<RefCell<MyStruct>> = Rc::downgrade(&ptr); // Create a weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to obtain shared pointer
        shared_ptr.borrow_mut().value = 20; // Modify data
        println!("Value: {}", shared_ptr.borrow().value);
    }
}