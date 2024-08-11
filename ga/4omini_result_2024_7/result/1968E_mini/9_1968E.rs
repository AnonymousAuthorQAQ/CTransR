use std::io::{self, Write};

fn solve(n: i32) {
    let mut t = n;

    // Printing first two lines
    println!("1 1");
    println!("2 1");
    t -= 2;

    // Printing when t is available
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Printing the remaining pattern
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear(); // Clear input buffer before reading in a new line
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        solve(n);
    }
}