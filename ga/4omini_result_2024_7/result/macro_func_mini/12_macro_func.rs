use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enter two integers (a and b):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();
    
    if values.len() < 2 {
        println!("Please provide two integers.");
        return;
    }

    let a = values[0];
    let b = values[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}