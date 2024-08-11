use std::io;

macro_rules! cppmax {
    ($x:expr, $y:expr) => {
        if $x + $y < 5 { $y } else { $x }
    };
}

fn main() {
    let mut input = String::new();
    println!("Enter two integers:");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.trim().split_whitespace();

    let a: i32 = nums.next().unwrap().parse().expect("Please enter a valid integer");
    let b: i32 = nums.next().unwrap().parse().expect("Please enter a valid integer");

    let result = cppmax!(a, b);
    println!("Result: {}", result);
}