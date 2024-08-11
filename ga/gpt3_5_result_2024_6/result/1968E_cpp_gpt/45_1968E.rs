use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
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
    let mut iter = stdin.lock().lines().map(|x| x.unwrap().parse().unwrap());

    let t: usize = iter.next().unwrap();

    for _ in 0..t {
        solve();
    }
}