use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let x: u64 = input.next().unwrap().parse().unwrap();
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

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid > 0
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}