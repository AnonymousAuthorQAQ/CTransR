use std::io::{self, BufRead};

fn solve(n: i32) {
    let mut t = n;
    
    // Print the fixed pairs
    println!("1 1");
    println!("2 1");
    t -= 2;
    
    // Handle the condition based on the remaining value of t
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }

    // Loop to print the remaining pairs
    for i in 3..(3 + t) {
        println!("1 {}", i);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        solve(n);
    }
}