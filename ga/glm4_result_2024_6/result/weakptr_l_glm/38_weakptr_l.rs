use std::sync::{Arc, Weak};
use std::fmt;

#[derive(Clone)]
struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Arc<Self> {
        Arc::new(MyStruct { value })
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value: {}", self.value)
    }
}

fn observe(weak: &Weak<MyStruct>) {
    if let Some(strong) = weak.upgrade() {
        println!("{}", strong);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let ptr1 = Arc::new(MyStruct::new(10));
    let weak_ptr1 = Arc::downgrade(&ptr1);

    {
        let ptr2 = ptr1.clone();
        println!("Inside inner scope");
        observe(&weak_ptr1);

        if let Some(strong) = weak_ptr1.upgrade() {
            strong.value = 20;
            println!("{}", strong);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    ptr1 = Arc::new(MyStruct::new(30));
    observe(&weak_ptr1);
}