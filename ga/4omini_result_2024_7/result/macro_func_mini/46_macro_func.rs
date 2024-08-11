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
    
    println!("Enter two integers:");

    // Read user input for two integers
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a valid integer"))
        .collect();

    // Unpack the two integers a and b
    let a = nums[0];
    let b = nums[1];

    // Calculate the result using the cppmax function
    let result = cppmax(a, b);
    
    // Print the result
    println!("Result: {}", result);
}