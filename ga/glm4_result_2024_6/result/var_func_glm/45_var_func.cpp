fn print_values<I>(values: I)
where
    I: IntoIterator,
    I::Item: std::fmt::Display,
{
    for value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(vec![10, 20, 30].into_iter());
}