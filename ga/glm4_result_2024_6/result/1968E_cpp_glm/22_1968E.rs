use std::io::{self, Read};

fn solve(n: usize) {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = input.trim().parse().unwrap();
        solve(n);
    }
}