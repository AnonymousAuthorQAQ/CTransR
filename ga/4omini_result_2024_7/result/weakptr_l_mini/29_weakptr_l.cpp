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

fn observe(weak_ptr: &Weak<MyStruct>) {
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        println!("Object is alive, value: {}", shared_ptr.borrow().value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let ptr1 = Rc::new(RefCell::new(MyStruct::new(10)));
    let weak_ptr1 = Rc::downgrade(&ptr1);

    {
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2 = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            shared_ptr.borrow_mut().value = 20;
            println!("Value: {}", shared_ptr.borrow().value);
        }
    } 

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Reset ptr1 (Drop it explicitly to simulate C++ reset)
    // When ptr1 goes out of scope, the original MyStruct will be dropped.
    // Since we use Rc we need to explicitly drop the shared pointer
    drop(ptr1);

    // At this point, weak_ptr1 is invalid as the original data is dropped.
    observe(&weak_ptr1); 
}