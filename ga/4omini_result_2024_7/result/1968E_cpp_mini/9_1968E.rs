use std::io::{self, Write};

fn solve() {
    let n: usize = read();
    let mut t = n;

    // Print the first two lines
    println!("1 1");
    println!("2 1");
    t -= 2;

    // If there's more to print, print the n, n line
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Print "1 i" for i ranging from 3 to 2 + t
    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn read() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<usize>().expect("Invalid input")
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        solve();
    }
}