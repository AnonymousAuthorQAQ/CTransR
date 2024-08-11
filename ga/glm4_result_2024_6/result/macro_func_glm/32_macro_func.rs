use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let numbers: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (a, b) = (numbers[0], numbers[1]);

    let result = max(a, b);
    println!("Result: {}", result);
}