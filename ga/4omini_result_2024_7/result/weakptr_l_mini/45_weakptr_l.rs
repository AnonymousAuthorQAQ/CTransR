use std::rc::{Rc, Weak};

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> Rc<MyStruct> {
        // Print the constructor message when a new instance is created
        println!("Constructor");
        Rc::new(MyStruct { value: v })
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        // Print the destructor message when an instance is dropped
        println!("Destructor");
    }
}

fn observe(weak_ptr: &Weak<MyStruct>) {
    match weak_ptr.upgrade() {
        Some(shared_ptr) => {
            println!("Object is alive, value: {}", shared_ptr.value);
        }
        None => {
            println!("Object has been destroyed");
        }
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
            shared_ptr.value = 20;
            println!("Value: {}", shared_ptr.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // When the last strong reference goes out of scope, the object will be dropped
    drop(ptr1);
    observe(&weak_ptr1);
}