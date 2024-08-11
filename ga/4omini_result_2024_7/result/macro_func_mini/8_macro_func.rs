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

    // Read the first integer
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Reset input string to read the second integer
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Calculate the result using cppmax function
    let result = cppmax(a, b);
    println!("Result: {}", result);
}