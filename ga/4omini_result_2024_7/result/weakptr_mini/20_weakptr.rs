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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to obtain shared pointer
        let mut value = shared_ptr.value; // Get a mutable reference to the value
        value = 20; // Modify data
        println!("Value: {}", value);
    }
}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct MyStruct {
    value: RefCell<i32>, // Use RefCell for interior mutability
}

impl MyStruct {
    fn new(v: i32) -> MyStruct {
        println!("Constructor");
        MyStruct {
            value: RefCell::new(v),
        }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10)); // Create shared pointer
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // Attempt to obtain shared pointer
        *shared_ptr.value.borrow_mut() = 20; // Modify data using RefCell
        println!("Value: {}", *shared_ptr.value.borrow());
    }
}