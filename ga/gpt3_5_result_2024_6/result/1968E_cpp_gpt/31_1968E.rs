use std::io::{self, BufRead};

fn solve(){
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
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
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}