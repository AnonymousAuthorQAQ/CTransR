use std::io::{self, Read};
use std::cmp::Ordering;
use std::num::gcd;

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid > mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _ in 0..t {
        let x: u64 = input.trim().parse().unwrap();
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