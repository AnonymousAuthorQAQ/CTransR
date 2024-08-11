use std::io::{self, Write};

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter two integers: ");
    io::stdout().flush().unwrap(); // Ensuring the prompt is printed before reading input

    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.split_whitespace()
                                 .map(|s| s.parse().unwrap())
                                 .collect();

    if numbers.len() == 2 {
        let a = numbers[0];
        let b = numbers[1];

        let result = cppmax(a, b);
        println!("Result: {}", result);
    } else {
        println!("Please enter exactly two integers.");
    }
}