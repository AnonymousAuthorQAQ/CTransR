fn print_values<T: std::fmt::Display>(n: usize, args: &[T]) {
    for value in args.iter() {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(3, &[10, 20, 30]);
}