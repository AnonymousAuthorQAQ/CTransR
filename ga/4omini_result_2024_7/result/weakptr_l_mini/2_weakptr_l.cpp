use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> Self {
        println!("Constructor");
        Self { value: v }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn observe(weak_ptr: &Weak<RefCell<MyStruct>>) {
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        let my_struct = shared_ptr.borrow();
        println!("Object is alive, value: {}", my_struct.value);
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
            let mut my_struct = shared_ptr.borrow_mut();
            my_struct.value = 20;
            println!("Value: {}", my_struct.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Drop ptr1 explicitly to see the destructor behavior
    // The destructor will be called for MyStruct when the last Rc reference (ptr1) is dropped.
    drop(ptr1);
    observe(&weak_ptr1);
}