use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        return y;
    } else {
        return x;
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter two numbers:");
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    
    let mut numbers = input.trim().split_whitespace();

    let a: i32 = numbers.next().unwrap().parse().expect("Invalid number");
    let b: i32 = numbers.next().unwrap().parse().expect("Invalid number");

    let result = cpp_max(a, b);
    
    println!("Result: {}", result);
}