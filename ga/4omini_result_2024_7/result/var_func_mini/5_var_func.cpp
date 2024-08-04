use std::fmt::Debug;

fn print_values<T: Debug>(values: T) {
    // Print the incoming values
    for value in values {
        println!("Value: {:?}", value);
    }
}

fn main() {
    // Pass a tuple to the print_values function
    print_values(vec![10, 20, 30]);
}