use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: i32, args: VaList) {
    for i in 0..n {
        let value: c_int = args.arg::<c_int>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(3, std::ffi::VaList::new(std::ptr::null()));
}