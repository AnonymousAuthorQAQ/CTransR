use std::io;
use std::cmp::Ordering;
use std::io::{self, BufRead};

fn can(x: i64, mid: i64) -> bool {
    let gcd = gcd(x, mid);
    gcd + mid <= x
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let x: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;
        
        while l <= r {
            let mid = (l + r) / 2;
            
            if can(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        
        println!("{}", ans);
    }
}