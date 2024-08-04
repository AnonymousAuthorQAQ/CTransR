use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    let t = n;
    println!("1 1");
    println!("2 1");

    let mut t_remaining = t - 2;
    if t_remaining > 0 {
        println!("{} {}", n, n);
        t_remaining -= 1;
    }

    for i in 3..(3 + t_remaining) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut test_cases_input = String::new();
    input.read_line(&mut test_cases_input).unwrap();
    let test_cases: usize = test_cases_input.trim().parse().unwrap();

    for _ in 0..test_cases {
        solve();
    }
}