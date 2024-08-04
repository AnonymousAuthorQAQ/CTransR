use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a number")).collect();
    let (a, b) = (values[0], values[1]);

    let result = if a + b < 5 { b } else { a };
    println!("Result: {}", result);
}