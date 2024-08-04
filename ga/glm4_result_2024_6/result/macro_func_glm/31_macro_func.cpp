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
    
    println!("Please enter two integers separated by a space:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut numbers = input.split_whitespace();
    let a: i32 = numbers.next().unwrap().parse().expect("Invalid number");
    let b: i32 = numbers.next().unwrap().parse().expect("Invalid number");
    
    let result = cppmax(a, b);
    println!("Result: {}", result);
}