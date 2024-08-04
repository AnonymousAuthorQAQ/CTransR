fn print_values(n: usize, args: &[i32]) {
    for &value in args.iter().take(n) {
        println!("Value: {}", value);
    }
}

fn main() {
    let args = [10, 20, 30];
    print_values(3, &args);
}