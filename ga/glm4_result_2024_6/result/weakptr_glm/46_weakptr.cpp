use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<RefCell<MyStruct>> {
        Rc::new(RefCell::new(MyStruct { value }))
    }

    fn set_value(&self, value: i32) {
        println!("Setting value to {}", value);
        self.value = value;
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_struct = Rc::downgrade(&my_struct);

    if let Some(my_struct) = weak_struct.upgrade() {
        my_struct.borrow_mut().set_value(20);
        println!("Value: {}", my_struct.borrow().value);
    }
}