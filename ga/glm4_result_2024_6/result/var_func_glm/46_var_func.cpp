fn print_values(n: usize, args: ...) {
    let mut args = std::va_list::from_slice(&args);
    for _ in 0..n {
        let value = args.next().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(3, 10, 20, 30);
}