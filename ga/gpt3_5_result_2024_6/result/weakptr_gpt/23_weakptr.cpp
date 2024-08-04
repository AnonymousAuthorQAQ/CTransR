use std::rc::Rc;
use std::cell::RefCell;

struct MyStruct {
    value: i32
}

impl MyStruct {
    fn new(v: i32) -> MyStruct {
        println!("Constructor");
        MyStruct { value: v }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let rc = Rc::new(RefCell::new(MyStruct::new(10)));
    let weak_rc = Rc::downgrade(&rc); // Create weak pointer
    if let Some(shared_rc) = weak_rc.upgrade() { // attempt to obtain shared_ptr
        shared_rc.borrow_mut().value = 20; // modify data
        println!("Value: {}", shared_rc.borrow().value);
    }
}