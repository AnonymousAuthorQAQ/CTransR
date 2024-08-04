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
    
    // Reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input.trim().split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly two numbers
    if nums.len() != 2 {
        eprintln!("Please enter exactly two integers.");
        return;
    }

    let a = nums[0];
    let b = nums[1];

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}