use std::ffi::VaList;
use std::os::raw::c_void;

extern "C" {
    fn ffi_print_values(n: i32, args: VaList<*const c_void>);
}

fn print_values(n: i32, args: VaList<*const c_void>) {
    unsafe {
        ffi_print_values(n, args);
    }
}

fn main() {
    let values = vec![10, 20, 30];
    let mut args = VaList::new();
    for &value in values.iter() {
        args.push(value as *const i32 as *const c_void);
    }
    print_values(values.len() as i32, args);
}