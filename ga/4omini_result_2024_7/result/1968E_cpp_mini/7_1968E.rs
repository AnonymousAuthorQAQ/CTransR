use std::io::{self, BufRead};

fn solve(n: usize) {
    let mut t = n;
    
    // Print the first two pairs
    println!("1 1");
    println!("2 1");
    t -= 2;

    // If t is not zero, print the pair (n, n)
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Print remaining pairs (1, i) where i ranges from 3 to 3 + t - 1
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read the value of n for each test case
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        solve(n);
    }
}