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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter an integer"))
        .collect();
    
    if numbers.len() == 2 {
        let a = numbers[0];
        let b = numbers[1];
        let result = cppmax(a, b);
        println!("Result: {}", result);
    } else {
        println!("Please enter exactly two integers.");
    }
}