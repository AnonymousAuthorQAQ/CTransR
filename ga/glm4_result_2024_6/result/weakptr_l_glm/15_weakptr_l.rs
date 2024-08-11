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
    let strong = match weak.upgrade() {
        Some(s) => s,
        None => {
            println!("Object has been destroyed");
            return;
        }
    };
    println!("Object is alive, value: {}", strong.borrow().value);
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    {
        println!("Inside inner scope");
        observe(&weak_my_struct);

        let my_struct2 = Rc::clone(&my_struct);
        if let Some(s) = weak_my_struct.upgrade() {
            s.borrow_mut().value = 20;
            println!("Value: {}", s.borrow().value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    Rc::strong_count(&my_struct); // Resetting the Rc
    observe(&weak_my_struct);
}