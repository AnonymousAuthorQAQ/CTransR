use std::ffi::VaList;

fn print_values(args: VaList) {
    for i in 0..args.len() {
        let value = args.into_iter().nth(i).unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let args = VaList::new();
    args.push(10);
    args.push(20);
    args.push(30);
    
    print_values(args);
}