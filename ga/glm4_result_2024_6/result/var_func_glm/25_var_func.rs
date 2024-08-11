fn print_values<I>(args: I)
where
    I: IntoIterator<Item = i32>,
{
    for value in args {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(vec![10, 20, 30]);
}