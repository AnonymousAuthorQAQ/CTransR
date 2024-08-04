use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::VaList;

extern "C" {
    fn va_start(vl: &mut VaList, n: c_int);
    fn va_arg(vl: &mut VaList, ty: c_int) -> c_int;
    fn va_end(vl: &mut VaList);
}

fn print_values(n: c_int, args: VaList) {
    let mut vl = args;
    unsafe {
        va_start(&mut vl, n);
        for i in 0..n {
            let value = va_arg(&mut vl, c_int);
            println!("Value: {}", value);
        }
        va_end(&mut vl);
    }
}

fn main() {
    let args = VaList::default();
    print_values(3, args);
}