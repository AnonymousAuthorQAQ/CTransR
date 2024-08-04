use std::rc::Rc;
use std::rc::Weak;

struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(v: i32) -> Rc<Self> {
        Rc::new(Self { value: v })
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_struct = Rc::downgrade(&my_struct);

    if let Some(shared_struct) = weak_struct.upgrade() {
        shared_struct.value = 20;
        println!("Value: {}", shared_struct.value);
    }
}