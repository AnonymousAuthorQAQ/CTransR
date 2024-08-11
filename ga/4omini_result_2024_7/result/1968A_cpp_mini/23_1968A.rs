use std::io::{self, BufRead};
use num::integer::gcd;

fn can(mid: i64, x: i64) -> bool {
    // Check if gcd(x, mid) + mid is greater than zero
    gcd(x, mid) + mid > 0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let x: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            if can(mid, x) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        println!("{}", ans);
    }
}