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

fn observe(weak_struct: &Weak<RefCell<MyStruct>>) {
    let strong_struct = weak_struct.upgrade();
    match strong_struct {
        Some(strong_struct) => {
            println!("Object is alive, value: {}", strong_struct.borrow().value);
        }
        None => {
            println!("Object has been destroyed");
        }
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_struct = Rc::downgrade(&my_struct);

    {
        let my_struct = Rc::clone(&my_struct);
        println!("Inside inner scope");
        observe(&weak_struct);

        if let Some(my_struct) = weak_struct.upgrade() {
            my_struct.borrow_mut().value = 20;
            println!("Value: {}", my_struct.borrow().value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_struct);

    Rc::strong_count(&my_struct); // No need to reset as Rc handles its own lifetime and destruction
    observe(&weak_struct);
}