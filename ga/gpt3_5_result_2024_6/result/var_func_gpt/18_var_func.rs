use std::ffi::VaList;

fn print_values(n: i32, args: VaList) {
    for _ in 0..n {
        let value = args.arg::<i32>();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::from([10, 20, 30]);
    print_values(3, args);
}