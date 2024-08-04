use std::io::{self, Read};
use std::cmp::Ordering;

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid > 0
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: i64 = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let x: i64 = lines.next().unwrap().parse().unwrap();
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;
        while l <= r {
            let mid = (l + r) / 2;
            match can(x, mid).cmp(&true) {
                Ordering::Greater => {
                    ans = mid;
                    l = mid + 1;
                }
                Ordering::Less => {
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