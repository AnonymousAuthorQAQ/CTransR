fn print_values(n: usize, args: &[i32]) {
    for &value in args.iter() {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = [10, 20, 30];
    print_values(values.len(), &values);
}