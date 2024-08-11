use std::io;
use std::io::prelude::*;

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter two integers separated by space: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let parts: Vec<&str> = input.trim().split(' ').collect();
    let a: i32 = parts[0].parse().expect("Invalid number");
    let b: i32 = parts[1].parse().expect("Invalid number");
    
    let result = cppmax(a, b);
    println!("Result: {}", result);
}