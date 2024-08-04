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
    
    // Read user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Parse the input into integers
    let a: i32 = parts[0].parse().expect("Please enter a valid number");
    let b: i32 = parts[1].parse().expect("Please enter a valid number");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}