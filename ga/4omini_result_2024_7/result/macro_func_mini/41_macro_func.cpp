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

    println!("Please enter two integers:");

    // Read input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    // Check if we have exactly two integers
    if nums.len() != 2 {
        println!("Please enter exactly two integers.");
        return;
    }

    let a = nums[0];
    let b = nums[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}