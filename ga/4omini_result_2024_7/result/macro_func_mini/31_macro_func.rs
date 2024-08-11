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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    if nums.len() == 2 {
        let a = nums[0];
        let b = nums[1];
        
        let result = cppmax(a, b);
        println!("Result: {}", result);
    } else {
        println!("Please enter exactly two integers.");
    }
}