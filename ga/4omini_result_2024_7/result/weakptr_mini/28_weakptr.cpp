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

fn main() {
    // Create a shared pointer (Rc) to MyStruct
    let ptr = Rc::new(MyStruct::new(10));

    // Create a weak pointer from the shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr);

    // Attempt to obtain a strong reference from the weak pointer
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify data
        shared_ptr.value = 20;
        println!("Value: {}", shared_ptr.value);
    }
}