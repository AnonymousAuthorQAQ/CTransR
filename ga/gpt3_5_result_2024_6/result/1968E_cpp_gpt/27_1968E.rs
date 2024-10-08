use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let t = n;
    println!("1 1");
    println!("2 1");

    let mut t = t - 2;

    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..3+t {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let test_cases: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        solve();
    }
}