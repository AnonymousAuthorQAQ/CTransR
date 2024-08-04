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
    let ptr1 = MyStruct::new(10);
    let weak_ptr1 = Rc::downgrade(&ptr1);

    {
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2 = Rc::downgrade(&ptr2);
        if let Some(strong) = weak_ptr2.upgrade() {
            strong.borrow_mut().value = 20;
            println!("Value: {}", strong.borrow().value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    Rc::delete(&ptr1);
    observe(&weak_ptr1);
}