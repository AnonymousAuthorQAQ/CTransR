fn print_values(values: Vec<i32>) {
    for value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    let args = vec![10, 20, 30];
    print_values(args);
}