use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter two integers:");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter a valid number"))
        .collect();

    if nums.len() < 2 {
        println!("Not enough numbers provided.");
        return;
    }

    let a = nums[0];
    let b = nums[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}