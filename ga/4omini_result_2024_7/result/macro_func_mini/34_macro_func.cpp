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

    // Read two integers from standard input
    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter a valid number"))
        .collect();

    // Ensure we have exactly two numbers
    if numbers.len() != 2 {
        eprintln!("Please enter exactly two integers.");
        return;
    }

    let a = numbers[0];
    let b = numbers[1];

    // Calculate the result using cppmax function
    let result = cppmax(a, b);

    // Print the result
    println!("Result: {}", result);
}