use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let t = n - 2;

    println!("1 1");
    println!("2 1");

    if t > 0 {
        println!("{} {}", n, n);
        if t > 1 {
            for i in 3..=n {
                println!("1 {}", i);
            }
        }
    }
}

fn main() {
    let t: usize = 1; // Assuming only one test case for now

    for _ in 0..t {
        solve();
    }
}