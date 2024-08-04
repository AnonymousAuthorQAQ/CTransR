use std::ffi::VaList;
use std::ffi::c_void;

fn print_values(n: u32, args: VaList) {
    for _i in 0..n {
        let value = args.arg::<i32>();
        println!("Value: {}", value.unwrap());
    }
}

fn main() {
    let first_argument = VaList::start(args1);

    print_values(3, first_argument);
}