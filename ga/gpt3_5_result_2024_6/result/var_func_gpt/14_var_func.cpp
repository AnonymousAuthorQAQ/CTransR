use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: c_int, args: VaList) {
    for i in 0..n {
        let value = args.arg::<c_int>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from(10).from(20).from(30);
    print_values(3, args);
}