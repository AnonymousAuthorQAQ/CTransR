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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(shared_ptr) = weak_ptr.upgrade() { // attempt to obtain shared_ptr
        // Modify data through shared_ptr
        let mut data = Rc::get_mut(&mut shared_ptr).unwrap();
        data.value = 20;
        println!("Value: {}", data.value);
    } else {
        println!("Weak pointer is expired");
    }
}