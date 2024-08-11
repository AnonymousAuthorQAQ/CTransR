use std::io::{self, BufRead};

fn solve() {
    let n = read_line_as_int();
    let t = n;
    println!("1 1");
    println!("2 1");
    let mut t = t - 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..=3 + t {
        println!("1 {}", i);
    }
}

fn read_line_as_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let t = read_line_as_int();
    for _ in 0..t {
        solve();
    }
}