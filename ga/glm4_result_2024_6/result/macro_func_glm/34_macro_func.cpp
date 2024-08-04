use std::io;

fn main() {
    // Read input
    print!("Enter two numbers: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    
    let a: i32 = iter.next().unwrap().parse().expect("Invalid number");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid number");

    // Calculate result using the CPPMAX logic
    let result = if (a + b) < 5 {
        b
    } else {
        a
    };

    // Output the result
    println!("Result: {}", result);
}