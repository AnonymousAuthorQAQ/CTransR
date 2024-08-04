use std::io;

fn main() {
    println!("Please enter two integers separated by space:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a valid integer"))
        .collect();

    let a = numbers[0];
    let b = numbers[1];

    let result = if a + b < 5 { b } else { a };
    println!("Result: {}", result);
}