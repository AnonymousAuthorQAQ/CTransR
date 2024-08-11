use std::io::{self, BufRead};

fn solve() {    
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut t = n;
    
    println!("1 1");
    println!("2 1");
    t -= 2;
    
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines = reader.lines();
    let t: usize = lines.clone().count(); // Count the number of test cases from input

    for _ in 0..t {
        solve();
    }
}