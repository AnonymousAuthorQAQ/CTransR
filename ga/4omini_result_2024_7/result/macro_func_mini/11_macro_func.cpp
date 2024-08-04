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

    // Read two integers from standard input
    println!("Enter two integers:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Call the cppmax function
    let result = cppmax(a, b);
    
    // Output the result
    println!("Result: {}", result);
}