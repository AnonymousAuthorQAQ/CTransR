use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> Rc<RefCell<MyStruct>> {
        Rc::new(RefCell::new(MyStruct { value: v }))
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