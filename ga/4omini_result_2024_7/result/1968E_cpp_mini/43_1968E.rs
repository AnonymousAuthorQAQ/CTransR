use std::io::{self, BufRead};

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
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        // Read n for each test case
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        solve(n);
    }
}