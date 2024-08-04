use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a number"))
        .collect();

    let (a, b) = (values[0], values[1]);

    let result = if (a + b) < 5 { b } else { a };

    println!("Result: {}", result);
}