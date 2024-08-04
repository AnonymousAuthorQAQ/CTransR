fn print_values<T: std::fmt::Display>(args: &[T]) {
    for arg in args {
        println!("Value: {}", arg);
    }
}

fn main() {
    let args = [10, 20, 30];
    print_values(&args);
}