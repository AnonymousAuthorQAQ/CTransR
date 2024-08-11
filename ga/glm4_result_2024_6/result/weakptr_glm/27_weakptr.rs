use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
    let weak_ref = Rc::downgrade(&my_struct);

    if let Some(shared_ref) = weak_ref.upgrade() {
        shared_ref.borrow_mut().value = 20;
        println!("Value: {}", shared_ref.borrow().value);
    } else {
        println!("No shared reference available");
    }
}