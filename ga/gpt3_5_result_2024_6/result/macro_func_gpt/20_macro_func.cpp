use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        return y;
    } else {
        return x;
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input.trim().split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let a = numbers[0];
    let b = numbers[1];

    let result = cppmax(a, b);
    
    println!("Result: {}", result);
}