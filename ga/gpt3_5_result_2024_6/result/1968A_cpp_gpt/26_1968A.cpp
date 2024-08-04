use std::io::{self, prelude::*};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid == true
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut t = String::new();
    input.read_line(&mut t).unwrap();
    let t: i64 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut x_str = String::new();
        input.read_line(&mut x_str).unwrap();
        let x: i64 = x_str.trim().parse().unwrap();

        let (mut l, mut r) = (1, x - 1);
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