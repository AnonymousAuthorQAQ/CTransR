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

    // Read input from the user
    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim() // Remove any leading/trailing whitespace
        .split_whitespace() // Split the input string by whitespace
        .filter_map(|s| s.parse().ok()) // Parse the substrings into integers
        .collect();

    if numbers.len() != 2 {
        println!("Please enter exactly two integers.");
        return;
    }

    let a = numbers[0];
    let b = numbers[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}