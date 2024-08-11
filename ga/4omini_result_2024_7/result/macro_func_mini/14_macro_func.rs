use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a valid number"))
        .collect();

    let (a, b) = (numbers[0], numbers[1]);

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}