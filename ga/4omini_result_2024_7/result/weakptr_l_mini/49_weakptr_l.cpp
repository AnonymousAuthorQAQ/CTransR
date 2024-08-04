use std::sync::{Arc, Weak};
use std::thread;

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
    let ptr1 = Arc::new(MyStruct::new(10));
    let weak_ptr1 = Arc::downgrade(&ptr1);

    {
        let ptr2 = Arc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2 = Arc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            shared_ptr.value = 20;  // Update value
            println!("Value: {}", shared_ptr.value);
        }
    } // ptr2 will go out of scope here, but ptr1 is still alive.

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Now resetting ptr1 will cause the destructor to be called.
    drop(ptr1);
    observe(&weak_ptr1); // Should indicate that the object has been destroyed.
}