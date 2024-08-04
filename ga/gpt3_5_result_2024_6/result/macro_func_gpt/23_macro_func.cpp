use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter two integers:");
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    
    let mut numbers = input.trim().split_whitespace();
    
    let a: i32 = numbers.next().unwrap().parse().unwrap();
    let b: i32 = numbers.next().unwrap().parse().unwrap();

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}