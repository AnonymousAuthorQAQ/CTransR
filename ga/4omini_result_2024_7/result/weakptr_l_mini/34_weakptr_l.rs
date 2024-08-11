use std::rc::{Rc, Weak};

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> MyStruct {
        println!("Constructor");
        MyStruct { value }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn observe(weak_ptr: &Weak<MyStruct>) {
    if let Some(strong_ptr) = weak_ptr.upgrade() {
        println!("Object is alive, value: {}", strong_ptr.value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let ptr1 = Rc::new(MyStruct::new(10));
    let weak_ptr1 = Rc::downgrade(&ptr1);

    {
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2 = Rc::downgrade(&ptr2);
        if let Some(strong_ptr) = weak_ptr2.upgrade() {
            strong_ptr.value = 20;
            println!("Value: {}", strong_ptr.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Resetting ptr1 to simulate the shared pointer going out of scope.
    drop(ptr1);
    observe(&weak_ptr1);
}