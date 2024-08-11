use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = parts[0].parse().expect("Please type a number!");
    let b: i32 = parts[1].parse().expect("Please type a number!");

    let result = if a + b < 5 { b } else { a };
    println!("Result: {}", result);
}