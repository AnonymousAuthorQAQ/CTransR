use std::ffi::VaList;

fn print_values(n: i32, args: VaList) {
    for i in 0..n {
        let value: i32 = args.arg();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::new();
    args.add::<i32>(10);
    args.add::<i32>(20);
    args.add::<i32>(30);
    
    print_values(3, args);
}