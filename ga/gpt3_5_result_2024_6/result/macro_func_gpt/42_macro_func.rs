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
    io::stdin().read_line(&mut input).unwrap();
    let mut numbers = input.trim().split(" ");
    let a: i32 = numbers.next().unwrap().parse().unwrap();
    let b: i32 = numbers.next().unwrap().parse().unwrap();

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}