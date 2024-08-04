fn print_values<T: std::fmt::Display>(n: usize, values: &[T]) {
    for value in values.iter().take(n) {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = [10, 20, 30];
    print_values(3, &values);
}