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

    if let Some(shared_ptr) = weak_ptr.upgrade() {
        // attempt to obtain shared_ptr
        let mut value = Rc::get_mut(&mut shared_ptr.clone()).unwrap();
        value.value = 20; // modify data
        println!("Value: {}", value.value);
    } else {
        println!("The weak pointer is expired.");
    }
}
   use std::rc::{Rc, Weak};
   use std::cell::RefCell;

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
       let ptr: Rc<RefCell<MyStruct>> = Rc::new(RefCell::new(MyStruct::new(10)));
       let weak_ptr: Weak<RefCell<MyStruct>> = Rc::downgrade(&ptr); // Create weak pointer

       if let Some(shared_ptr) = weak_ptr.upgrade() {
           // attempt to obtain shared_ptr
           shared_ptr.borrow_mut().value = 20; // modify data
           println!("Value: {}", shared_ptr.borrow().value);
       } else {
           println!("The weak pointer is expired.");
       }
   }
   