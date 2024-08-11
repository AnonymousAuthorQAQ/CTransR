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

    println!("Enter two numbers:");
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().expect("Not a number")).collect();
    
    let result = cpp_max(numbers[0], numbers[1]);
    println!("Result: {}", result);
}