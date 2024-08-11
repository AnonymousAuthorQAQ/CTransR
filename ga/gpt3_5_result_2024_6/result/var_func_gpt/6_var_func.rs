use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: c_int, ap: &mut VaList) {
    for _i in 0..n {
        let value = ap.arg::<c_int>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let mut ap = VaList::new();
    ap.start(3);
    print_values(3, &mut ap);
}