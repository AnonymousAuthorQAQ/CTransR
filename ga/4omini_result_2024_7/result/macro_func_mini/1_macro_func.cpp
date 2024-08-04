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
    
    // Read inputs from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let inputs: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = inputs[0].parse().expect("Please enter a valid integer");
    let b: i32 = inputs[1].parse().expect("Please enter a valid integer");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}