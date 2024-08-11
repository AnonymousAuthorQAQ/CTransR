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
    // Create a strong (shared) pointer
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    
    // Create a weak pointer from the strong pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr);
    
    // Attempt to upgrade the weak pointer to a strong pointer
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // Modify data
        shared_ptr.value = 20; // Note: Rust does not allow direct modification without inner mutability
        println!("Value: {}", shared_ptr.value);
    }

    // The Rc/Weak pointers will automatically deallocate memory when they go out of scope
}