use std::ffi::VaList;
use std::os::raw::c_int;

fn print_values(n: c_int, args: &VaList) {
    for _ in 0..n {
        let value = args.arg::<c_int>();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from(10 as c_int)
        .unwrap()
        .with_arg(20 as c_int)
        .unwrap()
        .with_arg(30 as c_int)
        .unwrap();
    print_values(3, &args);
}