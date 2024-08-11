use std::ffi::VaList;
use std::os::raw::c_int;

extern "C" {
    fn print_values(n: c_int, ...);
}

fn main() {
    // declare the external C function that takes variable arguments
    // the actual C implementation of the function is not provided since it is not necessary for Rust part
    unsafe {
        print_values(3, 10, 20, 30);
    }
}