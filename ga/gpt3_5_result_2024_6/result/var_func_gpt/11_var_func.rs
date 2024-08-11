use std::os::raw::c_int;
use std::ffi::VaList;
use std::ffi::VaListImpl;

fn print_values(n: c_int, mut vl: VaList) {
    for i in 0..n {
        let value: c_int = vl.arg::<c_int>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let values = vec![10, 20, 30];
    let mut vl = VaListImpl::new();
    
    for &value in values.iter() {
        vl.add::<c_int>(value);
    }

    print_values(values.len() as c_int, vl);
}