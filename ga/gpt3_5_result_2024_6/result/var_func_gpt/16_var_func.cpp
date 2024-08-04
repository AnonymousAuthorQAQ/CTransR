use std::ffi::VaList;

fn print_values(n: i32, args: VaList) {
    for i in 0..n {
        let value: i32 = args.arg::<i32>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from(&[10, 20, 30]);
    print_values(3, args);
}