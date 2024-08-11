use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<RefCell<MyStruct>> {
        Rc::new(RefCell::new(MyStruct { value }))
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    if let Some(shared_my_struct) = weak_my_struct.upgrade() {
        shared_my_struct.borrow_mut().value = 20;
        println!("Value: {}", shared_my_struct.borrow().value);
    }
}