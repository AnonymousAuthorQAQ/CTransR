use std::os::raw::c_int;
use std::ffi::VaList;
use std::ffi::VaListImpl;

fn print_values(n: c_int, vl: VaListImpl) {
    let mut va_list = vl;
    for _ in 0..n {
        let value = va_list.arg::<c_int>();
        println!("Value: {}", value);
    }
}

fn main() {
    unsafe {
        let vl = VaListImpl::from(va_arg_list!(3, 10, 20, 30));
        print_values(3, vl);
    }
}