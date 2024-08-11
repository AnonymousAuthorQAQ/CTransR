use std::io::{self, BufRead};

fn solve(n: i32) {
    let mut t = n;
    
    // Always output the first two pairs
    println!("1 1");
    println!("2 1");
    t -= 2;

    // If t is greater than 0, output the n, n pair
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Output the remaining pairs
    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines();

    // Read the number of test cases
    let t: i32 = reader.next().unwrap().unwrap().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        let n: i32 = reader.next().unwrap().unwrap().parse().unwrap();
        solve(n);
    }
}