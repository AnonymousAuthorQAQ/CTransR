fn print_values<T: std::fmt::Display>(n: usize, args: &[T]) {
    for (i, value) in args.iter().enumerate() {
        if i < n {
            println!("Value: {}", value);
        } else {
            break;
        }
    }
}

fn main() {
    let values = [10, 20, 30];
    print_values(3, &values);
}