use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut parts = input.trim().split_whitespace();
    let a: i32 = parts.next().unwrap().parse().expect("Invalid input");
    let b: i32 = parts.next().unwrap().parse().expect("Invalid input");

    let result = if (a + b) < 5 { b } else { a };
    println!("Result: {}", result);
}