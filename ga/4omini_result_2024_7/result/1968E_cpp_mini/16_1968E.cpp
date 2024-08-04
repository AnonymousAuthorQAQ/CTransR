use std::io::{self, BufRead};

fn solve(n: usize) {
    let mut output = Vec::new();
    
    output.push("1 1".to_string());
    output.push("2 1".to_string());
    
    let mut t = n;
    t -= 2; // Account for the first two outputs
    
    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }
    
    for i in 3..(3 + t) {
        output.push(format!("1 {}", i));
    }
    
    // Print all output lines
    for line in output {
        println!("{}", line);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    let t: usize = lines[0].parse().unwrap(); // Number of test cases
    
    for i in 1..=t {
        let n: usize = lines[i].parse().unwrap(); // Size for the current test case
        solve(n);
    }
}