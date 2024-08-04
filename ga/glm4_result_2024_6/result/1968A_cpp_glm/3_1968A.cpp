use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::ops::Sub;
use std::str::FromStr;

fn gcd(x: u64, y: u64) -> u64 {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid < x
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line");
    let t: u32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let x: u64 = input.trim().parse().expect("Invalid input");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = l + (r - l) / 2;

            match can(x, mid).cmp(&true) {
                Ordering::Less => {
                    l = mid + 1;
                }
                Ordering::Greater => {
                    r = mid - 1;
                }
                Ordering::Equal => {
                    ans = mid;
                    l = mid + 1;
                }
            }
        }

        println!("{}", ans);
    }
}