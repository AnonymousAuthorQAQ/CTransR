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
    let strong_ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create strong reference
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&strong_ptr); // Create weak reference

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to upgrade weak to strong
        let mut value = Rc::get_mut(&mut strong_ptr).unwrap(); // Get mutable reference to the strong pointer
        value.value = 20; // Modify the value
        println!("Value: {}", value.value); // Print modified value
    }
} // MyStruct is automatically dropped (Destructor called) at the end of the main function.