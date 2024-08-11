use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<&str> = input.trim().split(' ').collect();
    let a: i32 = values[0].parse().unwrap();
    let b: i32 = values[1].parse().unwrap();

    let result = if a + b < 5 { b } else { a };
    println!("Result: {}", result);
}