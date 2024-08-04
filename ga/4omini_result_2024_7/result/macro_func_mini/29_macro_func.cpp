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
    println!("Enter two integers: ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Please enter exactly two integers");
        return;
    }

    let a: i32 = parts[0].parse().expect("Please enter a valid integer");
    let b: i32 = parts[1].parse().expect("Please enter a valid integer");

    let result = cppmax(a, b);
    println!("Result: {}", result);
}