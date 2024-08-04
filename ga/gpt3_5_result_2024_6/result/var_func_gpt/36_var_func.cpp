use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn print_values(n: i32, ...);
}

fn main() {
    unsafe {
        print_values(3, 10, 20, 30);
    }
}