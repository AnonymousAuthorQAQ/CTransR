use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: c_int, mut args: VaList) {
    for _ in 0..n {
        let value = args.arg::<c_int>();
        println!("Value: {}", value);
    }
}

fn main() {
    unsafe {
        let args = VaList::consume(3, 10, 20, 30);
        print_values(3, args);
    }
}