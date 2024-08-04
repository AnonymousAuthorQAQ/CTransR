use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Clone)]
struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value }))
    }
}

fn observe(weak: &Weak<RefCell<MyStruct>>) {
    let strong = weak.upgrade();
    if let Some(strong) = strong {
        println!("Object is alive, value: {}", strong.borrow().value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    {
        println!("Inside inner scope");
        observe(&weak_my_struct);

        let my_struct_inner = my_struct.clone();
        if let Some(my_struct_inner) = my_struct_inner.borrow_mut() {
            my_struct_inner.value = 20;
            println!("Value: {}", my_struct_inner.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    Rc::try_unwrap(my_struct).ok();
    observe(&weak_my_struct);
}