use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    let mut t = n;
    
    println!("1 1");
    println!("2 1");
    t -= 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..3+t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}