use std::io::{self, BufRead};

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i64 = -1;
        let mut i_: i64 = -1;
        let mut j: i64 = -1;
        let mut j_: i64 = -1;
        
        let mut input = lines.next().unwrap();
        let mut nums = input.split_whitespace().map(|s| s.parse().unwrap());
        let n: usize = nums.next().unwrap();
        let k: usize = nums.next().unwrap();
        let a: usize = nums.next().unwrap();
        let b: usize = nums.next().unwrap();
        let m = min(n, k);
        let mut a_: usize = a;
        let mut b_: usize = b;
        
        let mut arr: Vec<i32> = Vec::with_capacity(n + 1);
        let mut aa: Vec<i32> = Vec::with_capacity(n + 1);
        
        input = lines.next().unwrap();
        let mut nums = input.split_whitespace().map(|s| s.parse().unwrap());
        for _ in 1..=n {
            arr.push(nums.next().unwrap());
        }
        
        input = lines.next().unwrap();
        let mut nums = input.split_whitespace().map(|s| s.parse().unwrap());
        for _ in 1..=n {
            aa.push(nums.next().unwrap());
        }
        
        for x in 1..=m {
            p1 = maxl(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a] as usize;
            p2 = maxl(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
            p2_ += aa[b] as i64;
            b = arr[b] as usize;
        }
        
        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}