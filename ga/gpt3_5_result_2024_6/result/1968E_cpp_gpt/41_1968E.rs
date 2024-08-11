use std::io::{self, BufRead};

fn solve(n: i64) {
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
    let mut buffer = String::new();
    
    stdin.lock().read_line(&mut buffer).unwrap();
    let t: i32  = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let n: i64 = buffer.trim().parse().unwrap();
        solve(n);
    }
}