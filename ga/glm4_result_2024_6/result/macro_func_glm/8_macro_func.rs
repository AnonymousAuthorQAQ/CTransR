use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();

    // Read two integers from the standard input
    println!("Enter two integers: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let a: i32 = parts.next().unwrap().parse().expect("Please enter a valid integer");
    let b: i32 = parts.next().unwrap().parse().expect("Please enter a valid integer");

    // Use the max function to determine the maximum value
    let result = max(a, b);

    // Print the result to the standard output
    println!("Result: {}", result);
}