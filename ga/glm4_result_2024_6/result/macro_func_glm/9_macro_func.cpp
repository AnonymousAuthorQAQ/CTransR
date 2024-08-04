use std::io;

fn main() {
    // Rust does not support macros like C++, so we implement a function instead
    fn cppmax(x: i32, y: i32) -> i32 {
        if x + y < 5 {
            y
        } else {
            x
        }
    }

    println!("Please enter two integers separated by a space: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut numbers = input.trim().split_whitespace();
    let a: i32 = numbers.next().unwrap().parse().expect("Invalid number");
    let b: i32 = numbers.next().unwrap().parse().expect("Invalid number");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}