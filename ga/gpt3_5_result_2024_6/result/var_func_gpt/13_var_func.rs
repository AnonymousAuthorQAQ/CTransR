use std::ffi::VaList;
use std::os::raw::{c_int};

fn print_values(n: i32, args: VaList) {
    for i in 0..n {
        let value = unsafe { args.arg::<c_int>() };
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from([&10 as *const c_int, &20 as *const c_int, &30 as *const c_int]);
    print_values(3, args);
}