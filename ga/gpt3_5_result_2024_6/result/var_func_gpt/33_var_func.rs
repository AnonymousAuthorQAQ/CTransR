use std::ffi::VaList;
use std::ffi::VaListImpl;

fn print_values(n: i32, vl: &VaListImpl) {
    for i in 0..n {
        let value: i32 = vl.arg();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = vec![10, 20, 30];
    let mut va_list = VaList::new(args.iter().map(|&x| x).collect());
    
    print_values(3, va_list.as_va_list());
}