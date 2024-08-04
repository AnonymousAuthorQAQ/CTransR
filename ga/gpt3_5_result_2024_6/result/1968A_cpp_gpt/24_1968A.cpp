use std::io::{self, Write};
use std::cmp::Ordering;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let x: u64 = input.trim().parse().unwrap();

        let mut l: u64 = 1;
        let mut r: u64 = x - 1;
        let mut ans: u64 = 1;

        while l <= r {
            let mid = (l + r) / 2;
            match can(x, mid) {
                true => {
                    ans = mid;
                    l = mid + 1;
                }
                false => {
                    r = mid - 1;
                }
            }
        }
        println!("{}", ans);
    }
}