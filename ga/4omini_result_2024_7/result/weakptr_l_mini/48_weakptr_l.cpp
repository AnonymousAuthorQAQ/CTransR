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
    match weak_ptr.upgrade() {
        Some(shared_ptr) => {
            println!("Object is alive, value: {}", shared_ptr.borrow().value);
        }
        None => {
            println!("Object has been destroyed");
        }
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
    } // `ptr2` goes out of scope here

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // When reset is called, we drop the last strong reference (ptr1).
    // We need to manually drop `ptr1` as Rust does not have a reset method that works like in C++
    drop(ptr1);
    observe(&weak_ptr1);
}