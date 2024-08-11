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

fn main() {
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create a shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create a weak pointer from the shared pointer

    // Attempt to obtain a strong reference from the weak pointer
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        shared_ptr.value = 20; // Modify data
        println!("Value: {}", shared_ptr.value);
    }
}