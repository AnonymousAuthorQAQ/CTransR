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

fn observe(weak_ptr: &Weak<RefCell<MyStruct>>) {
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        let value = shared_ptr.borrow().value;
        println!("Object is alive, value: {}", value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let ptr1: Rc<RefCell<MyStruct>> = Rc::new(RefCell::new(MyStruct::new(10)));
    let weak_ptr1: Weak<RefCell<MyStruct>> = Rc::downgrade(&ptr1);

    {
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2: Weak<RefCell<MyStruct>> = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            shared_ptr.borrow_mut().value = 20; // Change value to 20
            println!("Value: {}", shared_ptr.borrow().value);
        }
    } // ptr2 goes out of scope here

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Reset ptr1 by dropping it (it goes out of scope)
    drop(ptr1);
    observe(&weak_ptr1);
}