use std::io;
use std::cmp::Ordering;
use std::io::{self, Read};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid > mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i64>().unwrap();
    for _ in 0..t {
        let x = lines.next().unwrap().parse::<i64>().unwrap();
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