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
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.trim().split_whitespace();

    let a: i32 = numbers.next().unwrap().parse().expect("Invalid input");
    let b: i32 = numbers.next().unwrap().parse().expect("Invalid input");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}