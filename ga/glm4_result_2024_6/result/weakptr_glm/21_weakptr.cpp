struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Box<MyStruct> {
        println!("Constructor");
        Box::new(MyStruct { value })
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructor");
    }
}

fn main() {
    let ptr = MyStruct::new(10);
    let weak_ptr = Box::intoWeak(ptr);
    
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        shared_ptr.value = 20;
        println!("Value: {}", shared_ptr.value);
    }
}