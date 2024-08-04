use std::io::{self, BufRead};

fn solve(n: usize) {
    println!("1 1");
    println!("2 1");
    
    let mut t = n;
    t -= 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    let mut lines = handle.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap(); // First line -> number of test cases
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Each subsequent line -> n
        solve(n);
    }
}