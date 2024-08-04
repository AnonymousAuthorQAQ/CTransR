use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<RefCell<MyStruct>> {
        Rc::new(RefCell::new(MyStruct { value }))
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
    let ptr1 = MyStruct::new(10);
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

    Rc::forget(ptr1);
    observe(&weak_ptr1);
}