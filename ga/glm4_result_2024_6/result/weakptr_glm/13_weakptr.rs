use std::rc::Rc;
use std::rc::Weak;

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

fn main() {
    let my_struct = MyStruct::new(10);
    let weak_my_struct = Rc::downgrade(&my_struct);

    if let Some(my_struct) = weak_my_struct.upgrade() {
        my_struct.value = 20;
        println!("Value: {}", my_struct.value);
    }
}