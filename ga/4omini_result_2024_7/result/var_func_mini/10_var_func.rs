fn print_values(n: usize, values: &[i32]) {
    for &value in values.iter().take(n) {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(3, &[10, 20, 30]);
}