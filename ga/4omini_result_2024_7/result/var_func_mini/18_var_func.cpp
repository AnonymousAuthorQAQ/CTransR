fn print_values(values: &[i32]) {
    for &value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = vec![10, 20, 30];  // creating a Vec to hold the values
    print_values(&values);           // passing a reference to the slice of values
}