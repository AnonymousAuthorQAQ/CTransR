use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::VaList;

extern {
    fn va_start(args: *mut VaList, n: i32);
    fn va_arg(args: *mut VaList, type_: i32) -> i32;
    fn va_end(args: *mut VaList);
}

fn print_values(n: i32, args: *mut VaList) {
    unsafe {
        for i in 0..n {
            let value = va_arg(args, i32);
            println!("Value: {}", value);
        }
    }
}

fn main() {
    let mut vl: VaList = VaList::default();
    unsafe {
        va_start(&mut vl, 3);
    }
    
    print_values(3, &mut vl);

    unsafe {
        va_end(&mut vl);
    }
}