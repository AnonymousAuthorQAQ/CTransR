fn print_values<T: std::fmt::Display>(values: &[T]) {
    for value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(&[10, 20, 30]);
}