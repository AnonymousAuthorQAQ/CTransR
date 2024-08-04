use std::rc::{Rc, Weak};

struct MyStruct {
    value: i32,
}

// Rust automatically outputs messages for constructors and destructors for you
impl MyStruct {
    fn new(v: i32) -> MyStruct {
        println!("Constructor");
        MyStruct { value: v }
    }
}

// When MyStruct goes out of scope, the memory will be released
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create a `Rc` with MyStruct
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create a weak reference to it

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Try to obtain a strong reference
        // Modify data
        shared_ptr.value = 20; 
        println!("Value: {}", shared_ptr.value); // Print modified value
    }
}