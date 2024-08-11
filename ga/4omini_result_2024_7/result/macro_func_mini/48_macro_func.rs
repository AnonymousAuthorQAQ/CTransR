use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if numbers.len() < 2 {
        println!("Please enter two integers.");
        return;
    }
    
    let a = numbers[0];
    let b = numbers[1];

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}