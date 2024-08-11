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

    // Read input line
    println!("Enter two integers (a and b):");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input string into parts and parse them as integers
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    if values.len() != 2 {
        eprintln!("Please enter exactly two integers.");
        return;
    }

    let a = values[0];
    let b = values[1];

    // Calculate the result using the cppmax function
    let result = cppmax(a, b);
    
    // Print the result
    println!("Result: {}", result);
}