use std::io::{self, BufRead};

fn solve(n: usize) {
    let mut t = n;

    // Output the first two lines
    println!("1 1");
    println!("2 1");
    t -= 2;

    // If `t` is still greater than 0, print `n n`
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Output the remaining pairs `1 i`
    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();

    // Read number of test cases
    let t: usize = input.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        // For each test case, read `n`
        let n: usize = input.next().unwrap().unwrap().parse().unwrap();
        solve(n);
    }
}