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
        observe(&weak_ptr1);

        let weak_ptr2 = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            let value = &mut shared_ptr.value; // mutable reference to the value
            *value = 20; // change the value
            println!("Value: {}", *value);
        }
    } // ptr2 goes out of scope here

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Resetting `ptr1` to drop the reference and potentially destroy MyStruct
    // Note: In Rust, using `Rc::strong_count(&ptr1)` allows checking the count 
    // of strong pointers (for debug purposes), here we simply drop it.
    drop(ptr1);  // Manually drop the reference to cause the MyStruct to be dropped
    observe(&weak_ptr1);
}