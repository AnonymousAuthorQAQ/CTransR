use std::ffi::VaList;
use std::os::raw::c_void;

fn print_values(n: i32, args: VaList) {
    for i in 0..n {
        let value = args.arg::<i32>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from(10 as *const c_void);
    print_values(3, args);
}