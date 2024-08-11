use std::rc::{Rc, Weak};
use std::sync::Mutex;
use std::thread;
use std::borrow::Borrow;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<Mutex<Self>> {
        Rc::new(Mutex::new(Self { value }))
    }
}

fn observe(weak: &Weak<Mutex<MyStruct>>) {
    let locked = weak.upgrade();
    match locked {
        Some(shared) => {
            let value = shared.lock().unwrap().value;
            println!("Object is alive, value: {}", value);
        }
        None => println!("Object has been destroyed"),
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    {
        let my_struct = Rc::clone(&my_struct);
        println!("Inside inner scope");
        observe(&weak_my_struct);

        let mut my_struct = my_struct.lock().unwrap();
        my_struct.value = 20;
        println!("Value: {}", my_struct.value);
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    drop(my_struct);
    observe(&weak_my_struct);
}