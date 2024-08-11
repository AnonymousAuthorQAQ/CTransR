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
        // Create another shared pointer to the same instance
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        // Create a weak pointer from the new shared pointer
        let weak_ptr2: Weak<RefCell<MyStruct>> = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            shared_ptr.borrow_mut().value = 20; // Updating value
            let value = shared_ptr.borrow().value;
            println!("Value: {}", value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Resetting ptr1
    drop(ptr1);
    observe(&weak_ptr1);

    // The program ends here, and if ptr1 was the only owner, MyStruct will be dropped.
}