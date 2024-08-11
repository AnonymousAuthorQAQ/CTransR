use std::io::{self, Read, Write};

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let n: usize = lines.next().unwrap().parse().unwrap();
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
    let t: usize = read_line().trim().parse().expect("Invalid number");
    for _ in 0..t {
        solve();
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}