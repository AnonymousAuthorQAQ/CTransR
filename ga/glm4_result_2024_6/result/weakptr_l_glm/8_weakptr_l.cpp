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

fn observe(weak: &Option<&Box<MyStruct>>) {
    if let Some(shared) = weak {
        println!("Object is alive, value: {}", shared.value);
    } else {
        println!("Object has been destroyed");
    }
}

fn main() {
    let mut shared = Some(Box::new(MyStruct::new(10)));
    let weak = &shared;

    {
        let shared = shared.as_ref().unwrap();
        println!("Inside inner scope");
        observe(weak);

        let shared = shared.clone();
        if let Some(shared) = shared {
            shared.value = 20;
            println!("Value: {}", shared.value);
        }
    }

    println!("Outside inner scope");
    observe(weak);

    shared.take();
    observe(weak);
}