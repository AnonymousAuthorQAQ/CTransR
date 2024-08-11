use std::io;
use std::io::Write;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Failed to parse input");

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
    let t: i32 = 1; // Assuming only one test case for now

    for _ in 0..t {
        solve();
    }
}