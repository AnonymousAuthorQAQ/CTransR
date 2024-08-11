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
    let mut input = stdin.lock();
    
    let mut t = String::new();
    input.read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut n = String::new();
        input.read_line(&mut n).unwrap();
        let n: i32 = n.trim().parse().unwrap();
        solve(n);
    }
}