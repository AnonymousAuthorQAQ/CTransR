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

fn main() {
    let ptr = Rc::new(MyStruct::new(10)); // Create an Rc pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create a Weak pointer

    // Attempt to obtain the strong pointer from the weak pointer
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify data
        shared_ptr.value = 20;
        println!("Value: {}", shared_ptr.value);
    }
}