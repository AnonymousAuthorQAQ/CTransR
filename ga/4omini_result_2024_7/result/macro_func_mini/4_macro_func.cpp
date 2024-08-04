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

    // Read input from the user
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<&str> = input.trim().split_whitespace().collect();

    if numbers.len() != 2 {
        println!("Please enter exactly two integers.");
        return;
    }

    let a: i32 = numbers[0].parse().expect("Invalid input, not an integer");
    let b: i32 = numbers[1].parse().expect("Invalid input, not an integer");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}