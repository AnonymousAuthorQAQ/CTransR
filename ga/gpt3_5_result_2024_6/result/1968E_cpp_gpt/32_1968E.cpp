use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let n: usize = iterator.next().unwrap().parse().unwrap();
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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}