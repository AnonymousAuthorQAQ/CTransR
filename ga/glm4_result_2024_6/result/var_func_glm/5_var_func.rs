fn print_values<T: std::fmt::Display>(args: &[T]) {
    for value in args {
        println!("Value: {}", value);
    }
}

fn main() {
    let args = [10, 20, 30];
    print_values(&args);
}