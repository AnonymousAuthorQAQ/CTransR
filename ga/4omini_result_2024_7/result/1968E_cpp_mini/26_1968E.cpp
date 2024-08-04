use std::io::{self, Write};

fn solve(n: usize) {
    let mut output = Vec::new();
    
    // First two fixed outputs
    output.push(format!("1 1"));
    output.push(format!("2 1"));
    
    let mut t = n;
    t -= 2;
    
    // If t is greater than 0, output n n
    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }
    
    // Generate additional outputs for the remaining t
    for i in 3..(3 + t) {
        output.push(format!("1 {}", i));
    }

    // Print all outputs at once
    println!("{}", output.join("\n"));
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        solve(n);
    }
}