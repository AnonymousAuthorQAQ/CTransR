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

    println!("Enter two integers:");

    // Read the input from the user
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into two parts
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Please enter exactly two integers.");
        return;
    }

    // Parse the integers
    let a: i32 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input for the first integer.");
            return;
        }
    };
    
    let b: i32 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input for the second integer.");
            return;
        }
    };

    // Calculate the result using the cppmax function
    let result = cppmax(a, b);
    println!("Result: {}", result);
}