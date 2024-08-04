use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if (x + y) < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enter two integers:");

    // Read the input from the user
    io::stdin().read_line(&mut input). expect("Failed to read input");
    
    // Split the line into parts and parse them into integers
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace() // Split by whitespace
        .map(|s| s.parse().unwrap()) // Parse each part as an i32
        .collect();

    if let [a, b] = &numbers[..] {
        let result = cppmax(*a, *b);
        println!("Result: {}", result);
    } else {
        println!("Please enter exactly two integers.");
    }
}