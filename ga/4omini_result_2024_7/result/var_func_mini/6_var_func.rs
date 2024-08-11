use std::fmt::Display;

fn print_values<T: Display>(values: &[T]) {
    for value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = vec![10, 20, 30];
    print_values(&values);
}