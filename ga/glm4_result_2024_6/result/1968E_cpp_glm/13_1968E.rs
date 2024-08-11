use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    println!("1 1");
    println!("2 1");
    let mut t = n - 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..=n {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}