use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: c_int, vl: VaList) {
    for i in 0..n {
        let value = vl.arg::<c_int>();
        println!("Value: {}", value);
    }
}

fn main() {
    let vl = VaList::from(va_list!(10, 20, 30));
    print_values(3, vl);
}