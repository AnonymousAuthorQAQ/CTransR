use std::io::{self, Write};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    let mut t = n;

    // Output the predetermined lines
    println!("1 1");
    println!("2 1");
    t -= 2;

    // If remaining t is greater than 0, print `n n`
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Output remaining pairs
    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid number");

    for _ in 0..t {
        solve();
    }
}