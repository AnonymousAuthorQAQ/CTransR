use std::io::{self, BufRead};

fn solve(n: i32) {
    let t = n;
    println!("1 1");
    println!("2 1");
    
    let mut t_remaining = t - 2;
    
    if t_remaining > 0 {
        println!("{} {}", n, n);
        t_remaining -= 1;
    }
    
    for i in 3..3+t_remaining {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap().parse::<i32>().unwrap());
    
    let t = iterator.next().unwrap();

    for _ in 0..t {
        let n = iterator.next().unwrap();
        solve(n);
    }
}