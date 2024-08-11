use std::ffi::VaList;

fn print_values(n: i32, args: VaList) {
    for _ in 0..n {
        let value: i32 = args.arg::<i32>().expect("Error getting argument");
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::new();
    args.va_copy_start(3, 10, 20, 30);
    print_values(3, args);
}