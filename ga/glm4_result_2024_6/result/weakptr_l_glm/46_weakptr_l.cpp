use std::rc::{Rc, Weak};
use std::borrow::Borrow;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<MyStruct> {
        Rc::new(MyStruct { value })
    }

    fn observe(weak: &Weak<MyStruct>) {
        if let Some(shared) = weak.upgrade() {
            println!("Object is alive, value: {}", shared.borrow().value);
        } else {
            println!("Object has been destroyed");
        }
    }
}

fn main() {
    let rc1 = MyStruct::new(10);
    let weak_rc1 = Rc::downgrade(&rc1);

    {
        let rc2 = rc1.clone();
        println!("Inside inner scope");
        MyStruct::observe(&weak_rc1);

        let weak_rc2 = Rc::downgrade(&rc2);
        if let Some(shared) = weak_rc2.upgrade() {
            shared.borrow_mut().value = 20;
            println!("Value: {}", shared.borrow().value);
        }
    }

    println!("Outside inner scope");
    MyStruct::observe(&weak_rc1);

    Rc::from_raw(Rc::into_raw(rc1));
    MyStruct::observe(&weak_rc1);
}