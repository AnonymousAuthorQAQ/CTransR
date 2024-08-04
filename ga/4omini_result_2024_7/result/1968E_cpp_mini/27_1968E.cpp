use std::io::{self, Write};

fn solve(n: usize) {
    let mut output = vec![];
    
    output.push("1 1".to_string());
    output.push("2 1".to_string());
    
    if n > 2 {
        output.push(format!("{} {}", n, n));
    }
    
    let mut t = n.saturating_sub(2);
    
    for i in 3..3 + t {
        output.push(format!("1 {}", i));
    }
    
    // Print all outputs for the current test case
    for line in output {
        println!("{}", line);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();
        
        solve(n);
    }
}