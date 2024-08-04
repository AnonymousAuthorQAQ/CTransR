use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut line = String::new();
    handle.read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..=3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut line = String::new();
    handle.read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}