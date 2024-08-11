use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
    let ptr: Rc<RefCell<MyStruct>> = Rc::new(RefCell::new(MyStruct::new(10)));
    let weak_ptr: Weak<RefCell<MyStruct>> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to obtain Rc
        shared_ptr.borrow_mut().value = 20; // Modify data
        println!("Value: {}", shared_ptr.borrow().value);
    }
}