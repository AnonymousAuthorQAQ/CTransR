use std::io;
use std::io::{stdout, Write};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let t = 1; // Temporary fix for single test case
    // Read input for t test cases
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse().unwrap();
        let ps2: usize = iter.next().unwrap().parse().unwrap();
        
        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);
        
        // Read input for p and a arrays
        for _ in 0..n {
            p.push(iter.next().unwrap().parse().unwrap() - 1);
            a.push(iter.next().unwrap().parse().unwrap());
        }
        
        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);
        
        match ans1.cmp(&ans2) {
            std::cmp::Ordering::Equal => println!("Draw"),
            std::cmp::Ordering::Greater => println!("Bodya"),
            std::cmp::Ordering::Less => println!("Sasha"),
        }
    }
}