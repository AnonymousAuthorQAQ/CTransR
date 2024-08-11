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

fn observe(weak: &Weak<RefCell<MyStruct>>) {
    if let Some(shared) = weak.upgrade() {
        println!("Object is alive, value: {}", shared.borrow().value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak = Rc::downgrade(&my_struct);

    {
        let my_struct = Rc::clone(&my_struct);
        println!("Inside inner scope");
        observe(&weak);

        if let Some(shared) = weak.upgrade() {
            shared.borrow_mut().value = 20;
            println!("Value: {}", shared.borrow().value);
        }
    }

    println!("Outside inner scope");
    observe(&weak);

    drop(my_struct);
    observe(&weak);
}