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
    let strong = weak.upgrade();
    match strong {
        Some(strong) => {
            println!("Object is alive, value: {}", strong.borrow().value);
        }
        None => {
            println!("Object has been destroyed");
        }
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    {
        let my_struct = Rc::clone(&my_struct);
        println!("Inside inner scope");
        observe(&weak_my_struct);

        let strong = weak_my_struct.upgrade();
        if let Some(strong) = strong {
            strong.borrow_mut().value = 20;
            println!("Value: {}", strong.borrow().value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    drop(my_struct);
    observe(&weak_my_struct);
}