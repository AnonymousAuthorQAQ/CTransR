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
    let ptr = Rc::new(MyStruct::new(10)); // Create shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain shared_ptr
        shared_ptr.value = 20; // modify data
        println!("Value: {}", shared_ptr.value);
    }
}