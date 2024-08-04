use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    
    let n: usize = iter.next().unwrap().trim().parse().unwrap();
    
    let mut t = n;
    println!("1 1");
    println!("2 1");
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
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    
    let test_cases: usize = iter.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..test_cases {
        solve();
    }
}