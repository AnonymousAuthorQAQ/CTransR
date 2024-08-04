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

    fn modify_value(&mut self, new_value: i32) {
        self.value = new_value;
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_ref = Rc::downgrade(&my_struct);
    
    if let Some(shared_ref) = weak_ref.upgrade() {
        shared_ref.borrow_mut().modify_value(20);
        println!("Value: {}", shared_ref.borrow().value());
    } else {
        println!("Object has been deallocated.");
    }
}