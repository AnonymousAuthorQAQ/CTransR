use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::va_list;

extern "C" {
    fn va_start(vl: *mut va_list, n: c_int);
    fn va_arg(vl: *mut va_list, size: c_int) -> c_int;
    fn va_end(vl: *mut va_list);
}

fn print_values(n: c_int, ...) {
    let mut vl: va_list;
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
    print_values(3, 10, 20, 30);
}