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

fn observe(weak_ptr: &Weak<MyStruct>) {
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        println!("Object is alive, value: {}", shared_ptr.value);
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
        observe(&weak_ptr1); // Check if the object is alive

        let weak_ptr2 = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            shared_ptr.value = 20; // Modify the value
            println!("Value: {}", shared_ptr.value);
        }
    } // ptr2 goes out of scope here

    println!("Outside inner scope");
    observe(&weak_ptr1); // Check if the object is still alive

    // ptr1 goes out of scope and will be dropped here
    drop(ptr1); // Explicitly dropping ptr1 (optional)
    observe(&weak_ptr1); // Check if the object has been destroyed
}