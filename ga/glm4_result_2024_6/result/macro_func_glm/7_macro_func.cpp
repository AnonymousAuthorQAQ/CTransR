use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = values[0].parse().expect("Failed to parse a");
    let b: i32 = values[1].parse().expect("Failed to parse b");

    let result = if (a + b) < 5 { b } else { a };
    println!("Result: {}", result);
}