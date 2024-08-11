use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Failed to parse")).collect();

    let a = values[0];
    let b = values[1];

    let result = if a + b < 5 { b } else { a };

    println!("Result: {}", result);
}