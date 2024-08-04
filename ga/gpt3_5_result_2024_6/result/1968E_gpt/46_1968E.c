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
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|line| line.unwrap().parse::<i32>().unwrap());
    
    let t = iterator.next().unwrap();
    for _ in 0..t {
        let n = iterator.next().unwrap();
        solve(n);
    }
}