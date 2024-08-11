use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
    // Create a shared reference counter (Rc) to MyStruct
    let rc_ptr = Rc::new(RefCell::new(MyStruct::new(10))); 
    // Create a weak reference from the Rc
    let weak_ptr: Weak<RefCell<MyStruct>> = Rc::downgrade(&rc_ptr); 

    // Attempt to upgrade the weak pointer to a strong reference
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify data
        shared_ptr.borrow_mut().value = 20; 
        println!("Value: {}", shared_ptr.borrow().value); 
    }
}