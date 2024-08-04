use std::io::{self, Write};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut t = n;

    // Always print the first two pairs
    println!("1 1");
    println!("2 1");
    t -= 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Print remaining pairs
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases: usize = input.trim().parse().unwrap();

    for _ in 0..test_cases {
        solve();
    }
}