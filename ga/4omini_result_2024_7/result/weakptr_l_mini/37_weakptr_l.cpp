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
    let ptr1: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr1: Weak<MyStruct> = Rc::downgrade(&ptr1);

    {
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2: Weak<MyStruct> = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            shared_ptr.value = 20; // modifying the value
            println!("Value: {}", shared_ptr.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // When ptr1 goes out of scope, the value will be dropped.
    // Reset of ptr1 does not directly associate like C++. Use drop() 
    // function to explicitly tell our intent of destroying it.
    drop(ptr1); // This simulates invalidating the original pointer
    observe(&weak_ptr1);
}