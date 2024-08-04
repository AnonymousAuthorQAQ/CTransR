use std::io::{self, BufRead};

fn solve(n: i32) {
    let mut output = Vec::new();

    output.push("1 1".to_string());
    output.push("2 1".to_string());
    
    let mut t = n - 2; // Remaining lines to generate
    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }
    
    for i in 3..(3 + t) {
        output.push(format!("1 {}", i));
    }

    // Print all output for the current test case
    for line in output {
        println!("{}", line);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // For each test case, read n
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        solve(n);
    }
}