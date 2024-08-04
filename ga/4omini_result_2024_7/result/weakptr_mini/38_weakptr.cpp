use std::rc::{Rc, Weak};

struct MyStruct {
    value: i32,
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
    let ptr: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr: Weak<MyStruct> = Rc::downgrade(&ptr); // Create weak pointer

    if let Some(strong_ptr) = weak_ptr.upgrade() { // attempt to upgrade weak pointer
        let mut_ref = Rc::get_mut(&mut pointer.clone().into_inner()).unwrap();
        mut_ref.value = 20; // modify data through mutable ref
        println!("Value: {}", strong_ptr.value);
    }
    // narrow lifespan of strong pointer happens when destroyed Simplifying fresh rc as rust properly stages away№ outputs={}
}