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

    println!("Enter two numbers:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();

    let a = numbers[0];
    let b = numbers[1];

    let result = cppmax(a, b);
    println!("Result: {}", result);
}