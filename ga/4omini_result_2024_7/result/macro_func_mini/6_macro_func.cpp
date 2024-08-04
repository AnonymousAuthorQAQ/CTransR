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
        .split_whitespace()
        .map(|num| num.parse().expect("Please type a number!"))
        .collect();

    if nums.len() < 2 {
        eprintln!("Please enter two integers.");
        return;
    }

    let a = nums[0];
    let b = nums[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}