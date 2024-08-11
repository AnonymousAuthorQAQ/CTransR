use std::ffi::VaList;

fn print_values(n: i32, args: VaList) {
    for _ in 0..n {
        let value: i32 = args.arg::<i32>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::new();
    args.set_item::<i32>(10);
    args.set_item::<i32>(20);
    args.set_item::<i32>(30);
    
    print_values(3, args);
}