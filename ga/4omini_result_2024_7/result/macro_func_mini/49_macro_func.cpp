use std::io;

fn cppmax(x: i32, y: i32) -> i32 {
    if x + y < 5 {
        y
    } else {
        x
    }
}

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    
    println!("Enter two integers (a and b):");
    
    // Read first integer
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: i32 = input_a.trim().parse().expect("Please enter a valid integer");
    
    // Read second integer
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: i32 = input_b.trim().parse().expect("Please enter a valid integer");
    
    let result = cppmax(a, b);
    println!("Result: {}", result);
}