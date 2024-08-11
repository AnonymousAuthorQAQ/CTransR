use std::io::{self, Write};

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    
    print!("Enter two integers separated by a space: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    let numbers: Vec<i32> = input.trim().split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    
    let a = numbers[0];
    let b = numbers[1];
    
    let result = cpp_max(a, b);
    println!("Result: {}", result);
}