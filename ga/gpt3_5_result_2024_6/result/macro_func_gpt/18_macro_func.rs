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
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();

    let result = cppmax(nums[0], nums[1]);
    println!("Result: {}", result);
}