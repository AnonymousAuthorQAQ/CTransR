use std::rc::Rc;
use std::rc::Weak;
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
        let borrowed = shared.borrow();
        println!("Object is alive, value: {}", borrowed.value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let shared = MyStruct::new(10);
    let weak = Rc::downgrade(&shared);

    {
        let shared = Rc::clone(&shared);
        println!("Inside inner scope");
        observe(&weak);

        let shared = Rc::clone(&shared);
        if let Some(shared) = weak.upgrade() {
            shared.borrow_mut().value = 20;
            println!("Value: {}", shared.borrow().value);
        }
    }

    println!("Outside inner scope");
    observe(&weak);

    // Destroy the last Rc, making the object invalid
    drop(shared);
    observe(&weak);
}