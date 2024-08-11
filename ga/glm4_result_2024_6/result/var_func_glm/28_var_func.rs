fn print_values(n: i32, values: &[i32]) {
    for value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = [10, 20, 30];
    print_values(3, &values);
}