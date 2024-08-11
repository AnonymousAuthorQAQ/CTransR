use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::VaList;

extern {
    fn va_start(vl: *mut VaList, n: c_int);
    fn va_arg(vl: *mut VaList, ty: c_int) -> c_int;
    fn va_end(vl: *mut VaList);
}

fn print_values(n: c_int, ...) {
    let mut vl: VaList = VaList::default();
    unsafe {
        va_start(&mut vl, n);
        for _ in 0..n {
            let value = va_arg(&mut vl, c_int);
            println!("Value: {}", value);
        }
        va_end(&mut vl);
    }
}

fn main() {
    print_values(3, 10, 20, 30);
}