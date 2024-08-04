use std::io;

fn cpp_max(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = numbers[0].parse().expect("Invalid input");
    let b: i32 = numbers[1].parse().expect("Invalid input");

    let result = cpp_max(a, b);
    println!("Result: {}", result);
}