use std::rc::{Rc, Weak};
use std::ops::Deref;

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self { value })
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn observe(weak: &Weak<MyStruct>) {
    match weak.upgrade() {
        Some(shared) => {
            println!("Object is alive, value: {}", shared.value);
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
        let my_struct2 = Rc::clone(&my_struct);
        println!("Inside inner scope");
        observe(&weak_my_struct);

        let weak_my_struct2 = Rc::downgrade(&my_struct2);
        if let Some(shared) = weak_my_struct2.upgrade() {
            shared.value = 20;
            println!("Value: {}", shared.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_my_struct);

    // Rust does not have an explicit reset function like C++.
    // Instead, we drop the Rc explicitly.
    drop(my_struct);
    observe(&weak_my_struct);
}