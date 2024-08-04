use std::io::{self, BufRead};

fn solve(n: usize) {
    let mut t = n;
    
    // Output the first two fixed pairs
    println!("1 1");
    println!("2 1");
    
    t -= 2; // Decrease t by 2 since we already used two pairs
    
    // If there's remaining t, output (n, n)
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1; // Decrease t by 1
    
        // Output pairs of (1, i) for i from 3 to n
        for i in 3..=n {
            if t == 0 { break; }
            println!("1 {}", i);
            t -= 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    
    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        // Read the value of n for each test case
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        solve(n);
    }
}