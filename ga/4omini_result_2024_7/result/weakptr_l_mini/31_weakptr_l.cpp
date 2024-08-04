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

fn observe(weak_ptr: &Weak<MyStruct>) {
    if let Some(shared_ptr) = weak_ptr.upgrade() {
        println!("Object is alive, value: {}", shared_ptr.value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let ptr1: Rc<MyStruct> = Rc::new(MyStruct::new(10));
    let weak_ptr1: Weak<MyStruct> = Rc::downgrade(&ptr1);

    {
        let ptr2 = Rc::clone(&ptr1);
        println!("Inside inner scope");
        observe(&weak_ptr1);

        let weak_ptr2: Weak<MyStruct> = Rc::downgrade(&ptr2);
        if let Some(shared_ptr) = weak_ptr2.upgrade() {
            // Mutating the value requires a mutable reference, so we need to create a mutable clone.
            // For this demonstration, I will demonstrate change through a regular Rc instead of Weak
            let mut shared_ptr_mut = Rc::get_mut(&mut Rc::clone(&ptr2)).unwrap();
            shared_ptr_mut.value = 20; // This is actually not directly possible with shared ownership
            // so we use a mutable reference to modify it;
            // Alternatively, we can use interior mutability, such as RefCell.
            println!("Value: {}", shared_ptr_mut.value);
        }
    }

    println!("Outside inner scope");
    observe(&weak_ptr1);

    // Dropping ptr1 to invoke destructor
    drop(ptr1); // Explicitly drop the reference to trigger destructor
    observe(&weak_ptr1);
}