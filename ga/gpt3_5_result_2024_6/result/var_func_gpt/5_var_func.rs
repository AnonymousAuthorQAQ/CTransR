use std::ffi::VaList;
use std::os::raw::c_void;

fn print_values(n: i32, mut args: VaList) {
    for _ in 0..n {
        let value: i32 = args.arg::<i32>();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from(c_void::default());
    args.push::<i32>(10);
    args.push::<i32>(20);
    args.push::<i32>(30);
    
    print_values(3, args);
}