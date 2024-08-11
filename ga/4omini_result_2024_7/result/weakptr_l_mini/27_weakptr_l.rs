use std::sync::{Arc, Weak};

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
            shared_ptr.value = 20;
            println!("Value: {}", shared_ptr.value);
        }
    } // ptr2 goes out of scope here.

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Reset ptr1 to drop the strong reference.
    drop(ptr1);
    observe(&weak_ptr1);
}