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

fn observe(weak: &Weak<RefCell<MyStruct>>) {
    if let Some(shared) = weak.upgrade() {
        println!("Object is alive, value: {}", shared.borrow().value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    {
        let my_struct = Rc::clone(&my_struct);
        println!("Inside inner scope");
        observe(&weak_my_struct);

        let mut my_struct = Rc::make_mut(&my_struct);
        my_struct.value = 20;
        println!("Value: {}", my_struct.value);
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    Rc::try_unwrap(my_struct).unwrap();
    observe(&weak_my_struct);
}