use std::io::{self, BufRead};
use std::str::FromStr;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid < x
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