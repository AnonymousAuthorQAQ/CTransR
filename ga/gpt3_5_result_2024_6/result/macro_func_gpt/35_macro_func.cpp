use std::io::{self, Read};

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut numbers = input.trim().split_whitespace()
        .map(|num| num.parse().unwrap());
    
    let a = numbers.next().unwrap();
    let b = numbers.next().unwrap();

    let result = cpp_max(a, b);

    println!("Result: {}", result);
}