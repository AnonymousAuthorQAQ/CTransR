fn print_values(n: usize, values: &[i32]) {
    for &value in &values[..n] {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = vec![10, 20, 30];
    print_values(values.len(), &values);
}