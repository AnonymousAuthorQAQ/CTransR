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
    println!("Enter two integers:");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().expect("Invalid number"))
        .collect();

    let (a, b) = (nums[0], nums[1]);

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}