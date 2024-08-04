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
    
    // Reading input from the user
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Splitting the input and parsing as integers
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a number"))
        .collect();
    
    // Assuming the input has at least two numbers
    let a = numbers[0];
    let b = numbers[1];
    
    // Calling cppmax function and getting the result
    let result = cppmax(a, b);
    println!("Result: {}", result);
}