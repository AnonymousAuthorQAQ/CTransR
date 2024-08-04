use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let numbers: Vec<i32> = input.trim().split_whitespace()
        .map(|num| num.parse().expect("Invalid number"))
        .collect();
    
    let a = numbers[0];
    let b = numbers[1];
    
    let result = cpp_max(a, b);
    println!("Result: {}", result);
}

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}