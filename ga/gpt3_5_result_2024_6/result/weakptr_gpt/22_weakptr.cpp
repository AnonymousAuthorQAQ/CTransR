use std::rc::Rc;
use std::cell::RefCell;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> Self {
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
    let shared_ptr = Rc::new(RefCell::new(MyStruct::new(10)));
    let weak_ptr = Rc::downgrade(&shared_ptr);

    if let Some(shared_ref) = weak_ptr.upgrade() {
        shared_ref.borrow_mut().value = 20;
        println!("Value: {}", shared_ref.borrow().value);
    }
}