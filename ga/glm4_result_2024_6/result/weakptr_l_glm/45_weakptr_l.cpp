use std::rc::{Rc, Weak};
use std::fmt::Display;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<MyStruct> {
        Rc::new(MyStruct { value })
    }

    fn observe(weak: &Weak<MyStruct>) {
        match weak.upgrade() {
            Some(shared) => println!("Object is alive, value: {}", shared.value),
            None => println!("Object has been destroyed"),
        }
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    {
        let my_struct = Rc::clone(&my_struct);
        println!("Inside inner scope");
        MyStruct::observe(&weak_my_struct);

        if let Some(my_struct) = weak_my_struct.upgrade() {
            my_struct.value = 20;
            println!("Value: {}", my_struct.value);
        }
    }

    println!("Outside inner scope");
    MyStruct::observe(&weak_my_struct);

    drop(my_struct);
    MyStruct::observe(&weak_my_struct);
}