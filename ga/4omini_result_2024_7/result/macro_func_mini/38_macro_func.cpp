use std::io;

macro_rules! cppmax {
    ($x:expr, $y:expr) => {
        if $x + $y < 5 {
            $y
        } else {
            $x
        }
    };
}

fn main() {
    let mut input = String::new();

    // Read input from the user
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nums: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = nums[0].parse().expect("Please enter a number");
    let b: i32 = nums[1].parse().expect("Please enter a number");

    let result = cppmax!(a, b);
    println!("Result: {}", result);
}