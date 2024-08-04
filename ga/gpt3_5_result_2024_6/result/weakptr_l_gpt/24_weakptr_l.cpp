use std::cell::RefCell;
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

fn observe(weak_ptr: &Weak<RefCell<MyStruct>>) {
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        let borrowed = shared_ptr.borrow();
        println!("Object is alive, value: {}", borrowed.value);
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
            let mut borrowed = shared_ptr.borrow_mut();
            borrowed.value = 20;
            println!("Value: {}", borrowed.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    drop(ptr1);

    observe(&weak_ptr1);
}