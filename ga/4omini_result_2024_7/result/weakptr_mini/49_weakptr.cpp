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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create a shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create a weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to obtain a shared pointer
        let mut_value = Rc::get_mut(&mut shared_ptr).unwrap(); // Get mutable reference if unique
        if let Some(mut_value) = mut_value {
            mut_value.value = 20; // Modify data
            println!("Value: {}", mut_value.value);
        }
    }
}