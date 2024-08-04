use std::os::raw::c_int;
use std::ffi::VaList;

#[no_mangle]
extern "C" fn print_values(n: c_int, args: VaList) {
    for _ in 0..n {
        let value: c_int = args.arg::<c_int>();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::new();
    args.set_int(10);
    args.set_int(20);
    args.set_int(30);

    print_values(3, args);
}