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

fn observe(weak_struct: &Weak<RefCell<MyStruct>>) {
    if let Some(shared_struct) = weak_struct.upgrade() {
        println!("Object is alive, value: {}", shared_struct.borrow().value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    println!("Inside inner scope");
    observe(&weak_my_struct);

    {
        let mut my_struct_inner = Rc::clone(&my_struct);
        my_struct_inner.borrow_mut().value = 20;
        println!("Value: {}", my_struct_inner.borrow().value);
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    Rc::strong_count(&my_struct); // Ensure that my_struct is not used after this point
    observe(&weak_my_struct);
}