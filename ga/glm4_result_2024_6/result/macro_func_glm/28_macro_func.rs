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
    
    println!("Please enter two integers separated by a space:");
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    
    let a: i32 = iter.next().unwrap().parse().expect("Please enter a number");
    let b: i32 = iter.next().unwrap().parse().expect("Please enter a number");
    
    let result = cppmax(a, b);
    println!("Result: {}", result);
}