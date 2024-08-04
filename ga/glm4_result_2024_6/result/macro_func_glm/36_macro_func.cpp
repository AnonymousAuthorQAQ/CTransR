use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut numbers = input.trim().split_whitespace();
    let a: i32 = numbers.next().expect("Not enough numbers").parse().expect("Invalid number");
    let b: i32 = numbers.next().expect("Not enough numbers").parse().expect("Invalid number");

    let result = if (a + b) < 5 {
        b
    } else {
        a
    };

    println!("Result: {}", result);
}