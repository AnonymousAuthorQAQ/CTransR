use std::io::{self, Write};
use std::cmp::Ordering;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            match can(x, mid) {
                true => {
                    ans = mid;
                    l = mid + 1;
                },
                false => {
                    r = mid - 1;
                }
            }
        }

        println!("{}", ans);
    }
}