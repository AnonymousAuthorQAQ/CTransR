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

    // Getting user input
    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if values.len() < 2 {
        eprintln!("Please enter at least two integers.");
        return;
    }

    let a = values[0];
    let b = values[1];

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}