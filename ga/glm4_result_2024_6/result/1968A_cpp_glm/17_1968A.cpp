use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut t = String::new();
    stdin.read_line(&mut t).expect("Failed to read line");
    let t: u32 = t.trim().parse().expect("Input should be a number");

    for _ in 0..t {
        let mut x = String::new();
        stdin.read_line(&mut x).expect("Failed to read line");
        let x: u64 = x.trim().parse().expect("Input should be a number");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if gcd(&x, &mid) + mid <= x {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        write!(stdout, "{}\n", ans).expect("Failed to write");
    }
}

fn gcd(a: &u64, b: &u64) -> u64 {
    let mut x = *a;
    let mut y = *b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}