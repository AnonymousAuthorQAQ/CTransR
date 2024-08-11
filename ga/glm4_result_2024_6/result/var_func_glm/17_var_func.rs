fn print_values<T>(n: usize, values: T) 
where T: IntoIterator<Item = i32> {
    for value in values.into_iter().take(n) {
        println!("Value: {}", value);
    }
}

fn main() {
    let values = [10, 20, 30];
    print_values(3, &values[..]);
}