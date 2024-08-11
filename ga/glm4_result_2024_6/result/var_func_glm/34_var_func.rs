fn print_values<T: std::fmt::Display>(args: &[T]) {
    for value in args {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(&[10, 20, 30]);
}