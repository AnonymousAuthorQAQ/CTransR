fn print_values(n: usize, values: &[i32]) {
    for &value in values.iter().take(n) {
        println!("Value: {}", value);
    }
}

fn main() {
    // Example usage
    let values = vec![10, 20, 30];
    print_values(3, &values);
}