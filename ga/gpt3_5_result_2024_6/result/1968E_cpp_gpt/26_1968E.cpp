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
    let mut input = String::new();
    
    stdin.lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        solve(n);
    }
}