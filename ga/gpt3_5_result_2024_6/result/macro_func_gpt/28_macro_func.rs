use std::io::{self, BufRead};

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();

    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).unwrap();

    let mut numbers = input.trim().split_whitespace();
    let a: i32 = numbers.next().unwrap().parse().unwrap();
    let b: i32 = numbers.next().unwrap().parse().unwrap();

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}