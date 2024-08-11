use std::ffi::VaList;
use std::fmt;

fn print_values(n: usize, vl: VaList) {
    for _ in 0..n {
        let value: i32 = vl.arg::<i32>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let vl = std::ffi::VaList::start(1_i32);
    print_values(3, vl);
}