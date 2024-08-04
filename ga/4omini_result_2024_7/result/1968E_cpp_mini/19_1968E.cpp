use std::io::{self, Write};

fn solve(n: usize) {
    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;
    
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the number of test cases
    stdin.read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Input is not a number");
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");
        let n: usize = input.trim().parse().expect("Input is not a number");
        solve(n);
    }
}