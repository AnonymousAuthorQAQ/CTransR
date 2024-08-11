use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Clone)]
struct MyStruct {
    value: RefCell<i32>,
}

impl MyStruct {
    fn new(v: i32) -> Rc<Self> {
        Rc::new(Self { value: RefCell::new(v) })
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);
    
    if let Some(my_struct) = weak_my_struct.upgrade() {
        *my_struct.value.borrow_mut() = 20;
        println!("Value: {}", my_struct.value.borrow());
    }
}