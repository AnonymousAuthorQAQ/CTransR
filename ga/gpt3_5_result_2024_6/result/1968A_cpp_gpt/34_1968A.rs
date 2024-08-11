use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let t: i64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();

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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn can(x: i64, mid: i64) -> bool {
    return gcd(x, mid) == 1;
}