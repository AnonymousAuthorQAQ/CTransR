use std::io;

macro_rules! cppmax {
    ($x:expr, $y:expr) => {
        if ($x + $y) < 5 {
            $y
        } else {
            $x
        }
    };
}

fn main() {
    let mut input = String::new();

    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = numbers[0].parse().expect("Invalid input for a");
    let b: i32 = numbers[1].parse().expect("Invalid input for b");

    let result = cppmax!(a, b);
    println!("Result: {}", result);
}