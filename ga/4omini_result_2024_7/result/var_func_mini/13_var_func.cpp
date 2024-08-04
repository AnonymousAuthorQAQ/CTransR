fn print_values(values: &[i32]) {
    for &value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    let numbers = [10, 20, 30];
    print_values(&numbers);
}