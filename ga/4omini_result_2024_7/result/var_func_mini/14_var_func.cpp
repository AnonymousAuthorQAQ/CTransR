fn print_values(values: &[i32]) {
    for &value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(&[10, 20, 30]);
}