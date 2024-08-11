use std::io::{self, BufRead};

fn solve(n: i32) {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: i32 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: i32 = lines.next().unwrap().parse().unwrap();
        solve(n);
    }
}