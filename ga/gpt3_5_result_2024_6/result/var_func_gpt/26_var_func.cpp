use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: c_int, args: VaList) {
    for i in 0..n {
        let value: c_int = args.arg();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from(&[10 as c_int, 20 as c_int, 30 as c_int]);
    print_values(3, args);
}