use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut values = input.trim().split_whitespace();
    let a: i32 = values.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = values.next().unwrap().parse().expect("Failed to parse b");

    let result = if a + b < 5 { b } else { a };

    println!("Result: {}", result);
}