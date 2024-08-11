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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    // Attempt to upgrade the weak pointer to a strong reference (Rc)
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        let mut_value = Rc::make_mut(&mut shared_ptr); // Get mutable reference to modify value
        mut_value.value = 20; // Modify data
        println!("Value: {}", mut_value.value);
    }
}