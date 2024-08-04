use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();

    let (a, b) = (values[0], values[1]);

    let result = cppmax(a, b);
    println!("Result: {}", result);
}

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}